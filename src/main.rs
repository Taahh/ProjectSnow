#![deny(elided_lifetimes_in_paths)]

use std::{fs, thread};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs::{DirEntry, read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use normpath::PathExt;
use tide::{Body, log, Middleware, new, Redirect, Request, Response};
use tide::http::{Cookie, Mime};
use tide::prelude::Deserialize;

use crate::structs::structs::{Entry, RedirectEntry};

mod parsing;
mod structs;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

// #[actix_web::main]
// async fn start() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
//     // log::start();
//     let s = fs::read_to_string("test.toml").expect("Unable to parse");
//     let entry: HashMap<String, Entry> = parsing::parsing::parse(s);
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//
//     Ok(())
// }

fn main() {
    log::start();
    let s = fs::read_to_string("test.toml").expect("Unable to parse");
    let entry: HashMap<String, Entry> = parsing::parsing::parse(s);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        for (k, v) in entry {
            tokio::spawn(async move {
                let mut app = new();
                let entry = Entry::new(if v.file_directory().is_none() { None } else { v.file_directory() }, get(v.php_path()), if v.redirect().is_none() { None } else { v.redirect() });
                if !entry.redirect().is_none() {
                    let destination = entry.redirect().unwrap().destination();
                    app.at(v.redirect().unwrap().path().as_str()).get(Redirect::new(destination));
                }
                if !entry.file_directory().is_none() {
                    let directory = entry.file_directory().unwrap();
                    let mut args = directory.split(" ");
                    let mut path = args.next().unwrap();
                    let directory = args.next().unwrap();
                    let mut new_path = String::from(path);
                    if !new_path.ends_with("/") {
                        new_path = (new_path.to_string() + "/*");
                    } else if !new_path.ends_with("*") {
                        new_path = (new_path.to_string() + "*");
                    }
                    println!("Path of {:?} with dir {:?}", new_path, directory);
                    let read_dir = fs::read_dir(directory).expect("Unable to read directory");
                    let entries: Vec<DirEntry> = read_dir.map(|r| r.unwrap()).filter(|d| d.file_name().to_str().unwrap().to_string().to_lowercase().starts_with("index")).collect();
                    let parent_file = String::from(entries.first().unwrap().path().to_str().unwrap().to_owned());
                    let dir_again = String::from(directory);
                    app.at(&new_path).get(move |mut request: Request<_>| {
                        let tmp = parent_file.clone();
                        let tmp_dir = dir_again.clone();
                        async move {
                            println!("path: {:?}", request.url());
                            if !request.url().path().ends_with(".html") && !request.url().path().ends_with(".php") {
                                let parent_file_clone = tmp.clone();
                                if parent_file_clone.ends_with(".php") {
                                    let path = PathBuf::from(&tmp.clone()).as_path().normalize().unwrap().as_path().to_str().unwrap().replace("\\", "/");
                                    println!("Running command: {:?}", ("php -F '".to_owned() + path.clone().as_str() + "'").as_str());
                                    let cmd = "php -F '".to_owned() + path.clone().replace("\n", "").as_str() + "'";
                                    println!("Final Command: {}", cmd);
                                    println!("File name: {}", path);
                                    let output = Command::new("cmd")
                                        .env("GATEWAY_INTERFACE", "CGI/1.1")
                                        .env("SCRIPT_FILENAME", path.clone())
                                        .env("REDIRECT_STATUS", "200")
                                        .args(["/c", "php-cgi", "-f", path.clone().replace("\n", "").as_str()])
                                        .output().expect("Failed to run command");
                                    println!("output: {:?}", String::from_utf8(output.clone().stdout).unwrap());
                                    println!("input: {:?}", Command::new("cmd").args(["/c", "php-cgi", /*"-f",*/ path.clone().replace("\n", "").as_str()]));
                                    let php_output = String::from_utf8(output.stdout).unwrap();
                                    let php_output_again = php_output.clone();
                                    let mut output_args = php_output_again.split("\r\n\r\n");
                                    let header_args = output_args.next();
                                    let html = output_args.next();
                                    let mut body = Body::from_string(html.unwrap().to_string());
                                    body.set_mime(Mime::from_str("text/html;charset=utf-8").unwrap());
                                    let mut response = Response::from(body);
                                    let mut headers: Vec<String> = header_args.unwrap().split("\r\n").map(|s| s.to_string()).collect();
                                    println!("Headers: {:?}", headers);
                                    for x in headers {
                                        let mut key_value_pair: Vec<String> = x.split(": ").map(|s| s.to_string()).collect();
                                        let key = key_value_pair.get(0).unwrap().to_string();
                                        let value = key_value_pair.get(1).unwrap().to_string();
                                        response.insert_header(key.as_str(), value.as_str());
                                        if key.eq_ignore_ascii_case("Set-Cookie") {
                                            let cookie = Cookie::parse_encoded(value.clone()).unwrap();
                                            response.insert_cookie(cookie);
                                        }
                                    }
                                    return Ok(response);
                                }
                                return Ok(Response::from(Body::from_file(tmp.clone()).await?));
                            } else {
                                if request.url().path().ends_with(".php") {
                                    let mut path_args: Vec<String> = request.url().path().split("/").map(|s| s.to_string()).collect();
                                    let path_req: String = path_args[2..path_args.len()].join("/");
                                    let path = PathBuf::from(tmp_dir.clone().to_owned() + path_req.as_str()).as_path().normalize().unwrap().as_path().to_str().unwrap().replace("\\", "/");
                                    println!("Running command: {:?}", ("php -F '".to_owned() + path.clone().as_str() + "'").as_str());
                                    let cmd = "php -F '".to_owned() + path.clone().replace("\n", "").as_str() + "'";
                                    println!("Final Command: {}", cmd);
                                    println!("File name: {}", path);
                                    let output = Command::new("cmd")
                                        .env("GATEWAY_INTERFACE", "CGI/1.1")
                                        .env("SCRIPT_FILENAME", path.clone())
                                        .env("REDIRECT_STATUS", "200")
                                        .args(["/c", "php-cgi", "-f", path.clone().replace("\n", "").as_str()])
                                        .output().expect("Failed to run command");
                                    println!("output: {:?}", String::from_utf8(output.clone().stdout).unwrap());
                                    println!("input: {:?}", Command::new("cmd").args(["/c", "php-cgi", /*"-f",*/ path.clone().replace("\n", "").as_str()]));
                                    let php_output = String::from_utf8(output.stdout).unwrap();
                                    let php_output_again = php_output.clone();
                                    let mut output_args = php_output_again.split("\r\n\r\n");
                                    let header_args = output_args.next();
                                    let html = output_args.next();
                                    let mut body = Body::from_string(html.unwrap().to_string());
                                    body.set_mime(Mime::from_str("text/html;charset=utf-8").unwrap());
                                    let mut response = Response::from(body);
                                    let mut headers: Vec<String> = header_args.unwrap().split("\r\n").map(|s| s.to_string()).collect();
                                    println!("Headers: {:?}", headers);
                                    for x in headers {
                                        let mut key_value_pair: Vec<String> = x.split(": ").map(|s| s.to_string()).collect();
                                        let key = key_value_pair.get(0).unwrap().to_string();
                                        let value = key_value_pair.get(1).unwrap().to_string();
                                        response.insert_header(key.as_str(), value.as_str());
                                        if key.eq_ignore_ascii_case("Set-Cookie") {
                                            let cookie = Cookie::parse_encoded(value.clone()).unwrap();
                                            response.insert_cookie(cookie);
                                        }
                                    }
                                    return Ok(response);
                                }
                            }
                            let mut path_args: Vec<String> = request.url().path().split("/").map(|s| s.to_string()).collect();
                            let path_req: String = path_args[2..path_args.len()].join("/");
                            println!("Path requested: {:?}", path_req);
                            let dir: String = tmp_dir.clone().to_owned() + path_req.as_str();
                            println!("Directory Requested: {:?}", dir);
                            Ok(Response::from(Body::from_file(dir).await?))
                        }
                    }).serve_dir(&directory);
                }
                app.listen(k).await
            });
        };
    });
    loop {}
}

pub async fn nothing() {}

pub fn get<T>(option: Option<T>) -> Option<T> {
    if option.is_none() {
        return None;
    }
    return option;
}
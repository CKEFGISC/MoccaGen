// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


/*
import { invoke } from '@tauri-apps/api/tauri'
invoke('parse_token', { projectDirectory: "/Users/jimtsai/ytp/test" });
*/
 
use serde_json;
use std::io;
use std::fs::File;
use std::io::{Write};

const TOKEN_POSITION: &str = "token.json";
const GENERATOR_FORMAT: &str = "gen.cpp";
const GENERATOR_POSITION: &str = "gen";
const TEMPORARY_TOKEN: &str = "/Users/jimtsai/ytp/test";
const CPP_TEMPLATE_HEAD: &str = "
#include<bits/stdc++.h>
using namespace std;

signed main(){
    ios_base::sync_with_stdio(false);cin.tie(0);

";
 
//確認型別 debug用
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//把那個json object 變成string比較好輸出
fn printable<T: std::fmt::Display>(t: &T) -> String {
    t.to_string().replace("\"", "")
}

#[tauri::command]
fn parse_token(project_directory: &str) -> Result<String, String> {

    let token_path = format!("{}/{}", TEMPORARY_TOKEN, TOKEN_POSITION); //路徑 因為還沒創好先用我存在本地的
    let contents = match std::fs::read_to_string(&token_path) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };// 讀取json file

    let tokens_all: serde_json::Result<serde_json::Value> = serde_json::from_str(&contents); //parse json
    match tokens_all { //chatgpt 跟我說這樣編譯才會過 好像是因為parse的結果會有一個status code，成功或失敗都要接住，畢竟rust不能出錯嘛
        Ok(parsed_json) => { //parse 過了！
            println!("Parsed JSON: {:?}", parsed_json); //debug用
            if let Some(tokens_subtasks) = parsed_json["subtasks"].as_array() { //這邊也是要接error
                let gen_dir = format!("{}/gen", project_directory); // 找到檔案要寫去哪裡
                if !std::path::Path::new(&gen_dir).exists() {  // 如果檢測他是否已經有路徑 沒有的話就創
                    if let Err(e) = std::fs::create_dir(&gen_dir) {//chatgpt跟我說這樣編譯才會過 要接住資料夾創建失敗的可能性
                        return Err(format!("Failed to create directory '{}': {}", gen_dir, e)); //創檔失敗的報錯
                    }
                }
                for tokens in tokens_subtasks {// 處理每個subtask 分別開檔寫黨
                    let subtask_id = tokens["subtask_id"].as_str().unwrap_or_default();
                    let mut file_name = format!("{}/gen/sub{}gen.cpp", project_directory, subtask_id);
                    let mut file = match std::fs::File::create(&file_name) { 
                        Ok(file) => file,
                        Err(e) => return Err(format!("Failed to create file '{}': {}", file_name, e)),
                    };
                    let mut code = CPP_TEMPLATE_HEAD.to_string();
                    if let Some(tokens_array) = tokens["tokens"].as_array() {
                        for token_object in tokens_array {
                            let token_params = &token_object["object"];
                            let category = printable(&token_object["object"]["category"]);
                            let variable_name = printable(&token_object["id"]);
                            let class = printable(&token_params["class"]);
                            code = format!("{}gen_{} {}=gen_{}", code, category, variable_name, category);
                            // 處理那些constructer們
                            if category == "number".to_string() {
                                code = format!("{}(\"{}\")", code, class);
                            }
                            //end
                            
                            //處理attr

                            //end
                        }
                    } else {
                        return Err("Tokens array not found in JSON".to_string());
                    }
                    if let Err(e) = file.write_all(code.as_bytes()) {
                        return Err(format!("Failed to write to file '{}': {}", file_name, e));
                    }
                }
            } else {
                return Err("Subtasks not found in JSON".to_string());
            }
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Err(format!("Failed to parse JSON: {}", e));
        }
    }
    Ok("success".to_string())
}

/*

#[tauri::command]
fn parse_token(project_directory: &str) -> Result<String, String> {

    let token_path = format!("{}/{}", TEMPORARY_TOKEN, TOKEN_POSITION);
    let contents = match std::fs::read_to_string(&token_path) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };

    let tokens_all: serde_json::Result<serde_json::Value> = serde_json::from_str(&contents);
    match tokens_all {
        Ok(parsed_json) => {
            println!("Parsed JSON: {:?}", parsed_json);
            if let Some(tokens_subtasks) = parsed_json["subtasks"].as_array() {
                let gen_dir = format!("{}/gen", project_directory);
                if !std::path::Path::new(&gen_dir).exists() {
                    if let Err(e) = std::fs::create_dir(&gen_dir) {
                        return Err(format!("Failed to create directory '{}': {}", gen_dir, e));
                    }
                }
                for tokens in tokens_subtasks {
                    let subtask_id = tokens["subtask_id"].as_str().unwrap_or_default();
                    let mut file_name = format!("{}/gen/{}.cpp", project_directory, subtask_id);
                    let mut file = match std::fs::File::create(&file_name) {
                        Ok(file) => file,
                        Err(e) => return Err(format!("Failed to create file '{}': {}", file_name, e)),
                    };
                    let mut code = CPP_TEMPLATE_HEAD.to_string();
                    for token_object in tokens["tokens"].as_array() {
                        let token_params = &token_object["object"];
                        let category = printable(&token_object["object"]["category"]);
                        let variable_name = printable(&token_object["id"]);
                        let class = printable(&token_params["class"]);
                        code = format!("{}\ngen_{} {}=gen_{}", code, category, variable_name, category);
                        if category == "number" {
                            code = format!("{}(\"{}\")", code, class);
                        }
                    }
                    if let Err(e) = file.write_all(code.as_bytes()) {
                        return Err(format!("Failed to write to file '{}': {}", file_name, e));
                    }
                }
            } else {
                return Err("Subtasks not found in JSON".to_string());
            }
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Err(format!("Failed to parse JSON: {}", e));
        }
    }
    Ok("success".to_string())
}
*/

fn main() {
    use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
       .add_native_item(MenuItem::Copy)
       .add_item(CustomMenuItem::new("hide", "Hide"))
       .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![parse_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

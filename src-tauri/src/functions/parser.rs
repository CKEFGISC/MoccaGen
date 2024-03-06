use super::json;
use serde_json;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use tauri::utils::config::parse::parse_json;
//use json::{get_mcg_with_project_directory, parse};
//確認型別 debug用
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

//把那個json object 變成string比較好輸出
fn printable<T: std::fmt::Display>(t: &T) -> String {
  t.to_string().replace("\"", "")
}
const TOKEN_POSITION: &str = "token.json";

const GENERATOR_FORMAT: &str = "gen.cpp";
const GENERATOR_POSITION: &str = "gen";
const TEMPORARY_TOKEN: &str = "/Users/jimtsai/ytp/test";
const CPP_TEMPLATE_HEAD: &str = "
#include<bits/stdc++.h>
#include<assembler.h>
using namespace std;

signed main(){
    ios_base::sync_with_stdio(false);cin.tie(0);

";

#[tauri::command]
pub fn parse_token(token_path: &str, gen_path: &str, subtask_name: &str) -> Result<String, String> {
  //let token_path = format!("{}/{}", TEMPORARY_TOKEN, TOKEN_POSITION); //路徑 因為還沒創好先用我存在本地的
  println!("{}", token_path);
  let contents = match std::fs::read_to_string(&token_path) {
    Ok(contents) => contents,
    Err(e) => return Err(format!("Failed to read {}: {}", token_path, e)),
  }; // 讀取json file
  println!("{}", token_path);
  let tokens_all: serde_json::Result<serde_json::Value> = serde_json::from_str(&contents); //parse json
  match tokens_all {
    //chatgpt 跟我說這樣編譯才會過 好像是因為parse的結果會有一個status code，成功或失敗都要接住，畢竟rust不能出錯嘛
    Ok(parsed_json) => {
      //parse 過了！
      println!("{}", parsed_json);
      let subtask_id = parsed_json["subtask_id"].as_str().unwrap_or_default();
      let file_name = gen_path; //format!("{}/gen/{}_gen.cpp", project_directory, subtask_name);
      let mut file = match std::fs::File::create(&file_name) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to create file '{}': {}", file_name, e)),
      };

      let mut code = CPP_TEMPLATE_HEAD.to_string();

      if let Some(tokens_array) = parsed_json["tokens"].as_array() {
        for token_object in tokens_array {
          let token_params = &token_object["object"];
          let category = printable(&token_object["object"]["category"]);
          let variable_name = printable(&token_object["id"]);
          let class = printable(&token_params["class"]);
          // 處理array的template
          if category == "array".to_string() {
            code = format!("{}gen_{}<{}> {}=gen", code, category, class, variable_name);
            //code = format!("{}({}, {})", code, printable(token_params["len"]), printable(token_params["content"]));
          } else {
            code = format!("{}gen_{} {}=gen", code, category, variable_name);
          }
          //end

          //處理attr
          for (index, (key, value)) in token_params["attr"].as_object().unwrap().iter().enumerate()
          {
            if (key.starts_with('_')) {
              code = format!("{}{}(", code, key);
            } else {
              code = format!("{}.{}(", code, key);
            }
            println!("{}", printable(value));
            if let Some(attributes_array) = value.as_object() {
              for (i, attribute) in attributes_array.iter() {
                if (code.ends_with('(')) {
                  if (attribute == "integer" || attribute == "float" || i == "pattern") {
                    code = format!("{}{}", code, attribute);
                  } else {
                    code = format!("{}{}", code, printable(attribute));
                  }
                } else {
                  code = format!("{},{}", code, printable(attribute));
                }
              }
            }
            if code.ends_with(',') {
              code.pop();
            }
            code = format!("{})", code);
          }
          code = format!("{};\n", code);
          //end
        }
      } else {
        return Err("Tokens array not found in JSON".to_string());
      }
      if let Err(e) = file.write_all(code.as_bytes()) {
        return Err(format!("Failed to write to file '{}': {}", file_name, e));
      }
    }

    Err(e) => {
      eprintln!("Failed to parse JSON: {}", e);
      return Err(format!("Failed to parse JSON: {}", e));
    }
  }
  Ok("success".to_string())
}
// /Users/jimtsai/ytp/test/subtask1/token.json
#[tauri::command]
pub fn run_parser(mcg_path: &str) -> Result<String, String> {
  let project_path = json::get_project_directory_with_config_file(mcg_path);
  let config = json::parse(mcg_path.to_string()).unwrap();
  // Check if config is an array
  println!("{}", config);
  if let Some(subtasks) = config["subtasks"].as_array() {
    for subtask in subtasks {
      // Construct paths for token, generator, and subtask
      let token_path = format!(
        "{}//subtasks/{}",
        project_path,
        subtask["token"].as_str().unwrap_or("")
      );
      let gen_path = format!(
        "{}/subtasks/{}",
        project_path,
        subtask["generator"].as_str().unwrap_or("")
      );
      let subtask_path = format!(
        "{}/subtasks/{}",
        project_path,
        subtask["name"].as_str().unwrap_or("")
      );
      println!(
        "{} {} {} {}",
        token_path, project_path, subtask_path, gen_path
      );

      // Parse token for each subtask
      let res = parse_token(&token_path, &gen_path, &subtask_path);
      match res {
        Ok(goo) => println!("Success, {}", goo),
        Err(goo) => println!("Success, {}", goo),
      }
    }
  } else {
    return Err("Project configuration is not an array".to_string());
  }
  Ok("success".to_string())
}

#[tauri::command]
pub fn load_checker(project_path: &str) -> Result<String, String> {
  Ok("success".to_string())
}

#[tauri::command]
pub fn load_generator(project_path: &str) -> Result<String, String> {
  Ok("success".to_string())
}

#[tauri::command]
pub fn save_checker(project_path: &str) -> Result<String, String> {
  Ok("success".to_string())
}

#[tauri::command]
pub fn save_generator(project_path: &str) -> Result<String, String> {
  Ok("success".to_string())
}

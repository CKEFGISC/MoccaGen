// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


/*
import { invoke } from '@tauri-apps/api/tauri'
invoke('parse_token', { projectDirectory: "/Users/jimtsai/ytp/test" });
*/


mod functions;



use functions::{blockly, description, generator, init, parser, settings};


fn main() {
    use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("MochaGen", Menu::new());
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        // .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
          init::create_project,
          init::load_project,
          settings::load_settings,
          settings::save_settings,
          description::load_description,
          description::save_description,
          blockly::load_blockly,
          blockly::save_blockly,
          blockly::save_token,
          parser::parse_token,
          parser::run_parser,
          parser::load_checker,
          parser::load_generator,
          parser::save_checker,
          parser::save_generator,
          generator::generate_testdata
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

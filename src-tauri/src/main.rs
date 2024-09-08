// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::establish_conn;
use freq::{Br, Desc, MFreq};
use models::Db;
mod db;
mod models;
mod schema;
mod routs;
mod freq;
use std::sync::Mutex;
use routs::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let conn=Db{conn:Mutex::new(establish_conn())};
    let br=Br{d:Mutex::new(MFreq::new(5))};
    let des=Desc{d:Mutex::new(MFreq::new(5))};
    tauri::Builder::default()
        .manage(conn)
        .manage(br)
        .manage(des)
        .invoke_handler(tauri::generate_handler![
            greet,
            auth_o,
            create_customer,
            update_customer,
            get_all_customer,
            get_all_order,
            get_all_order_lists,
            get_orders_by_id,
            get_order_lists_by_id,
            place_order,
            update_payment
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

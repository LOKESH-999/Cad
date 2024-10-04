// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::establish_conn;
use freq::{Br, Desc, MFreq,Case,Pallets};
use models::Db;
mod db;
mod models;
mod schema;
mod routs;
mod freq;
use std::{sync::{atomic::AtomicI8, Condvar, Mutex}, thread, time::Duration as di};
use routs::*;
use models::{OIn, OList, BIn, BList};
use chrono::{NaiveDate,Duration};
mod email;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: Option<i32>) -> String {
    format!("Hello, {:?}! You've been greeted from Rust!", name)
}

#[derive(Debug)]
struct X{
    d:i32
}
impl X{
    fn new()->Self{
        X { d: 0 }
    }
    fn ch(mut self,i:i32)->X{
        self.d=i;
        self
    }
}

fn main() {
        // Create a sample OIn instance with test data
        let mh=email::send_mail::MailHandeler::init().unwrap();
        let x=Duration::days(21);
        let order_list = vec![
            OList {
                oil: "Olive Oil".to_string(),
                brand: "BrandA".to_string(),
                cases: 10,
                bottles: 100,
                cost: 200.0,
                n_weights: 50,
            },
            OList {
                oil: "Sunflower Oil".to_string(),
                brand: "BrandB".to_string(),
                cases: 5,
                bottles: 50,
                cost: 100.0,
                n_weights: 25,
            },
        ];
    
        let batch_data = Some(vec![
            BIn {
                total: 300.0,
                d_date: NaiveDate::from_ymd(2024, 12, 01),
                batch_list: vec![
                    BList {
                        oil: "Olive Oil".to_string(),
                        brand: "BrandA".to_string(),
                        n_weight: 50,
                        cases: 10,
                        bottles: 100,
                        cost: 200.0
                    },
                ],
            },
        ]);
    
        let order_input = OIn {
            cust_id: 123,
            m_batches: false,
            amount: 400.0,
            pending_amount: 100.0,
            order_list,
            batch_data: None,
            bottles:90,
            n_weights:1234,
            cases:90,
            msg: "Urgent".to_string(),
        };
        
    let conn=Db{conn:Mutex::new(establish_conn())};
    let br=Br{d:Mutex::new(MFreq::new(5))};
    let des=Desc{d:Mutex::new(MFreq::new(5))};
    let case=Case{d:Mutex::new(MFreq::new(5))};
    let pallet=Pallets{d:Mutex::new(MFreq::new(5))};
    println!("{:?}",pp(order_input, conn, br, des));
    // pp(order_input, conn, br, des);
    // pp(order_input, conn, br, des);
    // pp(order_input, conn, br, des);
    
    // println!("{:?}",br.d.l);
    let r=X::new();
    println!("{:?}",r);
    println!("{:?}",r.ch(34));

    println!("{:?}",thread::park_timeout(di::from_secs(15)));
    let x=AtomicI8::new(9);
    // println!("{:?}",x.)
    let conn=Db{conn:Mutex::new(establish_conn())};
    let br=Br{d:Mutex::new(MFreq::new(5))};
    let des=Desc{d:Mutex::new(MFreq::new(5))};
    tauri::Builder::default()
        .manage(conn)
        .manage(br)
        .manage(des)
        .manage(mh)
        .manage(case)
        .manage(pallet)
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
            send_mail,
            update_payments,
            get_desc_br
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

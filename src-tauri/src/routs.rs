use crate::db::*;
// use diesel::pg::PgConnection;
use tauri::State;
use tauri::Runtime; 
use diesel::result::Error;
use crate::models::*;
use crate::models::Db;

#[allow(unused)]
#[tauri::command]
pub fn auth_o<R: Runtime>(username:String,passwd:String,conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<bool, String> {
    let conn=&mut *(conn.conn.lock().unwrap());
    match auth(conn, username, passwd){
        Ok(_)=>Ok(true),
        Err(e)=>match e{
            Error::NotFound=>Ok(false),
            x=>Err(x.to_string())
        }
    }
}

#[allow(unused)]
#[tauri::command]
pub fn create_customer<R: Runtime>(data:CIn,conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Customer, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match add_customer(conn, data.into_c(chrono::Utc::now().naive_utc())){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
//   Ok(())
}

#[allow(unused)]
#[tauri::command]
pub fn update_customer<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<(), String> {
  todo!()
}

#[allow(unused)]
#[tauri::command]
pub fn get_all_customer<R: Runtime>(conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Vec<Customer>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_all_cust(conn) {
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
}

#[allow(unused)]
#[tauri::command]
pub fn get_all_order<R: Runtime>(conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<Order>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_all_orders(conn) {
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
}

#[allow(unused)]
#[tauri::command]
pub fn get_all_order_lists<R: Runtime>(conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<OrderList>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_all_order_list(conn) {
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
//   Ok(())
}

#[allow(unused)]
#[tauri::command]
pub fn get_orders_by_id<R: Runtime>(order_id:Option<i64>,cust_id:Option<i32>,conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<Order>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_order_by_id(conn, order_id, cust_id){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
//   Ok(())
}

#[allow(unused)]
#[tauri::command]
pub fn get_order_lists_by_id<R: Runtime>(id:i64,conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<OrderList>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_order_list_by_id(conn, id) {
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
}

#[allow(unused)]
#[tauri::command]
pub fn place_order<R: Runtime>(data:OIn,conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Order, String> {
    let (orders,order_list,batch)=data.split();
    todo!()
}
use crate::db::*;
use crate::freq::Br;
use crate::freq::Desc;
use diesel::dsl::Or;
use diesel::Connection;
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
}

#[allow(unused)]
#[tauri::command]
pub fn append_packages<R: Runtime>(data:Package,conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Package, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match add_packages(conn, data){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
}

#[allow(unused)]
#[tauri::command]
pub fn update_customer<R: Runtime>(conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<(), String> {
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
pub fn get_package<R: Runtime>(conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<Package>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    match get_packages(conn) {
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }

}

#[allow(unused)]
#[tauri::command]
pub fn get_desc_br<R: Runtime>(br:State<'_,Br>,des:State<'_,Desc>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(Vec<String>,Vec<String>), String> {
    Ok((
        br.d.lock().unwrap().get(),
        des.d.lock().unwrap().get()
    ))
}

#[allow(unused)]
#[tauri::command]
pub fn place_order<R: Runtime>(due_date:i64,data:OIn,conn:State<'_,Db>,mut br:State<'_,Br>,mut des:State<'_,Desc>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Order, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    let (orders,order_list,batch)=data.split(21);
    if (orders.m_batches==true && batch.is_none()) || (orders.m_batches==false && batch.is_some()){
        return Err("incorrect input fiels m_batches, and batches ".to_string());
    }
    br.d.lock().unwrap().add(order_list[0].brand.clone());
    des.d.lock().unwrap().add(order_list[0].oil.clone());
    match conn.transaction::<Order,Error,_>(|conn|{
        
        let order_result= match add_orders(conn, orders){
            Ok(x)=>x,
            Err(x)=>{return Err(x);}
        };
        let orderlist_result=match add_order_list(conn, order_result.order_id.unwrap(), order_list){
            Ok(x)=>x,
            Err(x)=>return Err(x)
        };
        if batch.is_some(){
            for i in batch.unwrap(){
                let (b,bl)=i.split(order_result.order_id.unwrap());
                let b_ret=match add_batch(conn, b.clone()) {
                    Ok(x)=>x,
                    Err(x)=>return Err(x)
                };
                match add_batch_list(conn, bl, b.order_id, b_ret.id.unwrap()) {
                    Ok(x)=>x,
                    Err(x)=>return Err(x)
                };
            }
        }    
        Ok(order_result) 
    }){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string()) 
    }
}

#[allow(unused)]
#[tauri::command]
pub fn update_payment<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    todo!();
  Ok(())
}

pub fn pp(data:OIn,conn:Db,br:Br,des:Desc) -> Result<Order, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    let (orders,order_list,batch)=data.split(21);
    if (orders.m_batches==true && batch.is_none()) || (orders.m_batches==false && batch.is_some()){
        return Err("incorrect input fiels m_batches, and batches ".to_string());
    }
    br.d.lock().unwrap().add(order_list[0].brand.clone());
    des.d.lock().unwrap().add(order_list[0].oil.clone());
    match conn.transaction::<Order,Error,_>(|conn|{
        
        let order_result= match add_orders(conn, orders){
            Ok(x)=>x,
            Err(x)=>{return Err(x);}
        };
        let orderlist_result=match add_order_list(conn, order_result.order_id.unwrap(), order_list){
            Ok(x)=>x,
            Err(x)=>return Err(x)
        };
        if batch.is_some(){
            for i in batch.unwrap(){
                let (b,bl)=i.split(order_result.order_id.unwrap());
                let b_ret=match add_batch(conn, b.clone()) {
                    Ok(x)=>x,
                    Err(x)=>return Err(x)
                };
                match add_batch_list(conn, bl, b.order_id, b_ret.id.unwrap()) {
                    Ok(x)=>x,
                    Err(x)=>return Err(x)
                };
            }
        }    
        Ok(order_result) 
    }){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string()) 
    }
}

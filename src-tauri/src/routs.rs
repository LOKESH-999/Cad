// use std::fmt::format;

use crate::db::*;
use crate::freq::Br;
use crate::freq::Case;
use crate::freq::Desc;
use crate::freq::Pallets;
use chrono::Duration;
use diesel::Connection;
// use diesel::pg::PgConnection;
use tauri::State;
use tauri::Runtime; 
use diesel::result::Error;
use crate::models::*;
use crate::models::Db;
use crate::email::send_mail::MailHandeler;

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
pub fn get_brand<R: Runtime>(conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<String>, String> {
    let conn=&mut *(conn.conn.lock().unwrap());
    get_brands(conn).map_err(|x| x.to_string())
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
pub fn get_orders_by_id(order_id:String,cust_id:String,conn:State<'_,Db>) -> Result<Vec<Order>, String> {
    println!("{:?},{:?}",order_id,cust_id);
    let order_id=order_id.parse::<i64>().ok();
    let cust_id=cust_id.parse::<i32>().ok();
    let conn=&mut *conn.conn.lock().unwrap();
    println!("{:?},{:?}",order_id,cust_id);
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
pub fn get_batchinfo_by_batchid<R: Runtime>(batch_id:i64,conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(Batch,Vec<BatchList>), String> {
    let conn=&mut *conn.conn.lock().unwrap();
    // get_batchlist_by_id(conn, batch_id).map_err(|x| x.to_string())
    todo!()
}

#[allow(unused)]
#[tauri::command]
pub fn get_bids_by_orderid<R: Runtime>(order_id:i64,conn:State<'_,Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Vec<Option<i64>>, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    get_batchid_by_orderid(conn, order_id).map_err(|x| x.to_string())
}

#[allow(unused)]
#[tauri::command]
pub fn get_batches_by_orderid<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    todo!();
  Ok(())
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
pub fn get_desc_br<R: Runtime>(br:State<'_,Br>,des:State<'_,Desc>,mut case:State<'_,Case>,mut pallet:State<'_,Pallets>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(Vec<String>,Vec<String>,Vec<String>,Vec<String>), String> {
    Ok((
        br.d.lock().unwrap().get(),
        des.d.lock().unwrap().get(),
        case.d.lock().unwrap().get(),
        pallet.d.lock().unwrap().get(),
    ))
}

#[allow(unused)]
#[tauri::command]
pub fn place_order<R: Runtime>(due_date:i64,data:OIn,conn:State<'_,Db>,mut br:State<'_,Br>,mut des:State<'_,Desc>,mut case:State<'_,Case>,mut pallet:State<'_,Pallets>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Order, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    let (orders,order_list,batch)=data.split(21);
    if (orders.m_batches==true && batch.is_none()) || (orders.m_batches==false && batch.is_some()){
        return Err("incorrect input fiels m_batches, and batches ".to_string());
    }
    br.d.lock().unwrap().add(order_list[0].brand.clone());
    des.d.lock().unwrap().add(order_list[0].oil.clone());
    order_list.iter().map(|x|{
        pallet.d.lock().unwrap().add(x.bottles.to_string());
        case.d.lock().unwrap().add(x.cases.to_string());
    });
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
        // else{
        //     let batch=Batch{
        //         id:None,
        //         order_id:order_result.order_id.unwrap(),
        //         total:order_result.amount,
        //         d_date:(order_result.order_date+Duration::days(1)).into()
        //     };
        //     let batch_result=match add_batch(conn, batch) {
        //         Ok(x)=>x,
        //         Err(x)=>return Err(x)
        //     };
        //     let data=orderlist_result.into_iter().map(|x| x.into_blist()).collect::<Vec<BList>>();
        //     let batch_list_result=match add_batch_list(conn, data, batch_result.order_id, batch_result.id.unwrap()) {
        //         Ok(x)=>x,
        //         Err(x)=>return Err(x)
        //     };
            
        // }    
        Ok(order_result) 
    }){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string()) 
    }
}

#[allow(unused)]
#[tauri::command]
pub fn update_payments<R: Runtime>(orderid:i64,paid:f64,method:String,conn:State<Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Order, String> {
    let conn=&mut *conn.conn.lock().unwrap();
    let o=get_order_by_id(conn, Some(orderid), None)?;
    if o.len()<1 || o.len()>1{
        return Err("Nothing found".into())
    }
    let o=&o[0];
    let c=o.pending_amount-paid;
    // status 3 paid status 2 partial paid status 1 not paid status 4 only invoice
    let status=if c<0.0{
        return Err(format!("amount exeded the cap {} - {} < 0",o.pending_amount,paid).to_string());
    }else if c>0.0{
        2
    }else{
        3
    };
    update_orders(conn, orderid, o.m_batches, o.amount, c, status).map_err(|x|x.to_string())
}

#[allow(unused)]
#[tauri::command]
pub fn send_mail<R: Runtime>(mailid:String,subject:String,body:Option<String>,f_name:String,fd:Vec<u8>,mh:State<MailHandeler,'_>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    _=mh.send_bin_attachements(mailid, subject, body, f_name.into(),fd).map_err(|x| x.to_string())?;
    Ok(())
}

#[allow(unused)]
#[tauri::command]
pub fn send_auto_mail<R: Runtime>(custid:i32,subject:String,body:Option<String>,f_name:String,fd:Vec<u8>,mh:State<MailHandeler,'_>,conn:State<Db>,app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    let conn=&mut *conn.conn.lock().unwrap();
    let mailid=get_email_by_id(conn, custid).map_err(|x|{x.to_string()})?;
    for x in mailid{
        _=mh.send_bin_attachements(x, subject.clone(), body.clone(), f_name.clone(), fd.clone()).map_err(|x| x.to_string())?;
    }
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
        }else{
            let batch=Batch{
                id:None,
                order_id:order_result.order_id.unwrap(),
                total:order_result.amount,
                d_date:(order_result.order_date+Duration::days(1)).into()
            };
            let batch_result=match add_batch(conn, batch) {
                Ok(x)=>x,
                Err(x)=>return Err(x)
            };
            let data=orderlist_result.into_iter().map(|x| x.into_blist()).collect::<Vec<BList>>();
            let _batch_list_result=match add_batch_list(conn, data, batch_result.order_id, batch_result.id.unwrap()) {
                Ok(x)=>x,
                Err(x)=>return Err(x)
            };
            
        }    
        Ok(order_result) 
    }){
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string()) 
    }
}

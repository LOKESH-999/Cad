use crate::db::*;
use diesel::pg::PgConnection;
use tauri::State;
use tauri::Runtime; 
use diesel::result::Error;
use crate::models::Db;

#[tauri::command]
async fn auth_o<R: Runtime>(username:String,passwd:String,conn:State<'_,Db>,_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<bool, String> {
    let conn=&mut *(conn.conn.lock().unwrap());
    match auth(conn, username, passwd){
        Ok(_)=>Ok(true),
        Err(e)=>match e{
            Error::NotFound=>Ok(false),
            x=>Err(x.to_string())
        }
    }
}



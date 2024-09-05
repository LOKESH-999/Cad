use crate::{models::*, schema::batch};
use crate::schema::{auth, batch_list, customers, e_ids, order_list, orders, user_watchdog};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env; 
use chrono::{NaiveDate,NaiveDateTime,Utc};

pub fn establish_conn()->PgConnection{
    dotenv().ok();
    let url=env::var("DATABASE_URL").unwrap();
    PgConnection::establish(url.as_str()).unwrap()
}

pub fn auth(conn:&mut PgConnection,username:String,passwd:String)->Result<Auth,Error>{
    auth::dsl::auth
    .filter(auth::dsl::username.eq(username))
    .filter(auth::dsl::passwd.eq(passwd))
    .limit(1)
    .get_result::<Auth>(conn)
}

pub fn add_customer(conn:&mut PgConnection,val:Customer)->Result<Customer,Error>{
    diesel::insert_into(customers::table)
    .values(val)
    .returning(Customer::as_returning())
    .get_result(conn)
}

pub fn add_batch(conn:&mut PgConnection,order_id:i64,total:f64,d_date:NaiveDate)->Result<Batch,Error>{
    diesel::insert_into(batch::table)
    .values(Batch{
        id:None,
        order_id:order_id,
        total:total,
        d_date:d_date
    }).returning(Batch::as_returning())
    .get_result(conn)
}

pub fn add_batch_list(conn:&mut PgConnection,data:Vec<BList>,order_id:i64,batch_id:i64)->Result<Vec<BatchList>,Error>{
    diesel::insert_into(batch_list::table)
    .values(data.into_iter().map(|f| f.into_batch_list(order_id, batch_id)).collect::<Vec<BatchList>>())
    .returning(BatchList::as_returning())
    .get_results(conn)
} 

pub fn add_orders(conn:&mut PgConnection,val:Order)->Result<Order,Error>{
    diesel::insert_into(orders::table)
    .values(val)
    .returning(Order::as_returning())
    .get_result(conn)
    // todo!()
}
 
pub fn add_order_list(conn:&mut PgConnection,order_id:i64,val:Vec<OList>)->Result<Vec<OrderList>,Error>{
     diesel::insert_into(order_list::table)
     .values(val.into_iter().map(|x| x.into_order_list(order_id)).collect::<Vec<OrderList>>())
     .returning(OrderList::as_returning())
     .get_results(conn)
}

pub fn update_orders(conn:&mut PgConnection,order_id:i64,m_batches:bool,total:f64,pending:f64)->Result<Order,Error>{
    diesel::update(orders::table)
    .filter(orders::order_id.eq(order_id))
    .set((
        orders::m_batches.eq(m_batches),
        orders::amount.eq(total),
        orders::pending_amount.eq(pending)
    )).returning(Order::as_returning())
    .get_result(conn)
    // todo!()
}

pub fn update_batch(conn:&mut PgConnection,id:i64,total:f64,d_date:NaiveDate)->Result<Batch,Error>{
    diesel::update(batch::table)
    .filter(batch::id.eq(id))
    .set((
        batch::total.eq(total),
        batch::d_date.eq(d_date)
    )).returning(Batch::as_returning())
    .get_result(conn)
}

// pub fn update_batch_list(conn:&mut PgConnection,)

pub fn update_customer(id:i32,name:String,hst:String,address:String,phone_no:i64,email:String,conn:&mut PgConnection)->Result<Customer, Error>{
    diesel::update(customers::table)
    .filter(customers::cust_id.eq(id))
    .set((
        customers::name.eq(name),
        customers::hst.eq(hst),
        customers::address.eq(address),
        customers::phone.eq(phone_no),
        customers::primary_email.eq(email)
    )).returning(Customer::as_returning()).get_result(conn)
}
pub fn get_all_cust(conn:&mut PgConnection)->Result<Vec<Customer>,Error>{
    customers::dsl::customers.load::<Customer>(conn)
}

pub fn get_all_orders(conn:&mut PgConnection)->Result<Vec<Order>,Error>{
    orders::dsl::orders.load::<Order>(conn)
}

pub fn get_all_order_list(conn:&mut PgConnection)->Result<Vec<OrderList>,Error>{
    order_list::dsl::order_list.load::<OrderList>(conn)
}

pub fn get_order_by_id(conn:&mut PgConnection,id:Option<i64>,cust_id:Option<i32>)->Result<Vec<Order>,String>{
    if id.is_none() && cust_id.is_none(){
        return Err("both order_id and customer_id is none".to_string());
    }
    match if id.is_none(){
        orders::table.filter(orders::cust_id.eq(cust_id.unwrap())).load::<Order>(conn)
    }else  {
        orders::table.filter(orders::order_id.eq(id.unwrap())).load::<Order>(conn)
    }{
        Ok(x)=>Ok(x),
        Err(x)=>Err(x.to_string())
    }
}

pub fn get_order_list_by_id(conn:&mut PgConnection,id:i64)->Result<Vec<OrderList>,Error>{
    order_list::table.filter(order_list::order_id.eq(id)).load::<OrderList>(conn)
    // todo!()
}

pub fn orders_by_date(conn:&mut PgConnection,date:NaiveDate)->Result<Vec<Order>,Error>{
    orders::table.filter(diesel::dsl::date(orders::order_date).eq(date)).load::<Order>(conn)
}

pub fn get_email_by_id(conn:&mut PgConnection,cust_id:i32)->Result<Vec<String>,Error>{
    e_ids::table.filter(e_ids::cust_id.eq(cust_id)).select(e_ids::email).get_results::<String>(conn)
}

pub fn user_log(conn:&mut PgConnection,username:String,desc:String)->Result<usize,Error>{
    diesel::insert_into(user_watchdog::table)
    .values(UserWatchdog{
            id:None,
            username:username,
            date_time:Utc::now().naive_utc().into(),
            descs:desc    
    })
    .execute(conn)
}
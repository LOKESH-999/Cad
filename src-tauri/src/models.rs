// @generated automatically by Diesel CLI.
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono;
use std::sync::Mutex;

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = auth)]
pub struct Auth {
    pub username: String,
    pub passwd: String,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = batch)]
pub struct Batch {
    pub id: Option<i64>,
    pub order_id: i64,
    pub total: f64,
    pub d_date: chrono::NaiveDate,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = batch_list)]
pub struct BatchList {
    pub id: Option<i64>,
    pub batch_id: i64,
    pub order_id: i64,
    pub oil: String,
    pub brand: String,
    pub n_weight: i32,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
    pub msg: String,
    pub term: chrono::NaiveDate,
}
#[derive(Debug,Serialize,Deserialize)] 
pub struct BList {
    pub oil: String,
    pub brand: String,
    pub n_weight: i32,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
    pub msg: String,
    pub term: chrono::NaiveDate,
}
impl BList{
    pub fn into_batch_list(self,order_id:i64,batch_id:i64)->BatchList{
        BatchList{
            id:None,
            batch_id:batch_id,
            order_id:order_id,
            oil:self.oil,
            brand:self.brand,
            n_weight:self.n_weight,
            cases:self.cases,
            bottles:self.bottles,
            cost:self.cost,
            msg:self.msg,
            term:self.term
        }
    }
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = brands)]
pub struct Brand {
    pub brand: String,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub cust_id: Option<i32>,
    pub name: String,
    pub hst: String,
    pub address: String,
    pub primary_email: String,
    pub phone:i64,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CIn {
    pub name: String,
    pub hst: String,
    pub address: String,
    pub primary_email: String,
    pub phone:i64
}
impl CIn{
    pub fn into_c(self,date:chrono::NaiveDateTime)->Customer{
        Customer { cust_id: None, name: self.name, hst: self.hst, address: self.address, primary_email: self.primary_email, phone: self.phone, date: date }
    }
}


#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = e_ids)]
pub struct EId {
    pub email: String,
    pub cust_id: i32,
}

// #[derive(De)]
pub struct  Db{
    pub conn:Mutex<PgConnection>
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = order_list)]
pub struct OrderList {
    pub id: Option<i64>,
    pub order_id: i64,
    pub oil: String,
    pub brand: String,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
    pub n_weights: i32,
    pub msg: String,
    pub term: chrono::NaiveDate,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct OList{
    pub oil: String,
    pub brand: String,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
    pub n_weights: i32,
    pub msg: String,
    pub term: chrono::NaiveDate,
}

impl OList{
    pub fn into_order_list(self,order_id:i64)->OrderList{
        OrderList{
            id:None,
            order_id:order_id,
            oil:self.oil,
            brand:self.brand,
            n_weights:self.n_weights,
            cases:self.cases,
            bottles:self.bottles,
            cost:self.cost,
            msg:self.msg,
            term:self.term
        }
    }
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub order_id: Option<i64>,
    pub cust_id: i32,
    pub m_batches: bool,
    pub amount: f64,
    pub pending_amount: f64,
    pub order_date: chrono::NaiveDateTime,
    pub due_date: chrono::NaiveDate,
    pub status: i16,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = packages)]
pub struct Package {
    pub descs: String,
    pub bottel_in_pallet: i16,
    pub price: f32,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: Option<i64>,
    pub order_id: i64,
    pub amount: f64,
    pub method: String,
    pub time: chrono::NaiveDateTime,
    pub status: i16,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = shipping_details)]
pub struct ShippingDetail {
    pub ship_id: Option<i64>,
    pub carrier: String,
    pub tracking_no: String,
    pub eda: chrono::NaiveDate,
    pub cost: f32,
    pub n_weight: i32,
    pub address: String,
    pub batch_id: i64,
}

#[derive(Queryable,Selectable,Insertable,Debug,Serialize,Deserialize)]
#[diesel(table_name = user_watchdog)]
pub struct UserWatchdog {
    pub id: Option<i64>,
    pub username: String,
    pub date_time: chrono::NaiveDateTime,
    pub descs: String,
}

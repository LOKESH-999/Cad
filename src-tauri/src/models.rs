use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono::{self, Duration,Utc};
use std::sync::Mutex;
use diesel::pg::Pg;

#[derive(Queryable,Selectable,Insertable,QueryableByName,Debug,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = auth)]
pub struct Auth {
    pub username: String,
    pub passwd: String,
}

#[derive(Queryable,Selectable,Insertable,QueryableByName,Debug,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = brands)]
pub struct Brands{
    brand:String
}

#[derive(Queryable,Selectable,Insertable,QueryableByName,Debug,Serialize,Deserialize,Clone)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = batch)]
pub struct Batch {
    pub id: Option<i64>,
    pub order_id: i64,
    pub total: f64,
    pub d_date: chrono::NaiveDate,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct BIn{
    pub total:f64,
    pub d_date:chrono::NaiveDate,
    pub batch_list:Vec<BList>
}
impl BIn{
    pub fn split(self,order_id:i64)->(Batch,Vec<BList>){
        (
            Batch { id: None, order_id: order_id, total: self.total, d_date: self.d_date },
            self.batch_list
        )
    }
}
#[derive(Queryable,Selectable,Insertable,QueryableByName,Debug,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
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
}
#[derive(Debug,Serialize,Deserialize)] 
pub struct BList {
    pub oil: String,
    pub brand: String,
    pub n_weight: i32,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
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
        }
    }
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = brands)]
pub struct Brand {
    pub brand: String,
}

#[derive(Queryable,Selectable,Insertable,QueryableByName,Debug,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
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


#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = e_ids)]
pub struct EId {
    pub email: String,
    pub cust_id: i32,
}

// #[derive(De)]
pub struct  Db{
    pub conn:Mutex<PgConnection>
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
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
}
impl OrderList{
    pub fn into_blist(self)->BList{
        BList{
            oil:self.oil,
            brand:self.brand,
            n_weight:self.n_weights,
            cases:self.cases,
            bottles:self.bottles,
            cost:self.cost,
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct OList{
    pub oil: String,
    pub brand: String,
    pub cases: i32,
    pub bottles: i32,
    pub cost: f64,
    pub n_weights: i32,
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
        }
    }
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = orders)]
pub struct Order {
    pub order_id: Option<i64>,
    pub cust_id: i32,
    pub m_batches: bool,
    pub amount: f64,
    pub pending_amount: f64,
    pub order_date: chrono::NaiveDateTime,
    pub status: i16,
    pub msg: String,
    pub n_weight: i32,
    pub cases: i32,
    pub bottles: i32,
    pub term: chrono::NaiveDate
}

// impl OrderList{
//     pub fn into_batch_list(self)->BatchList{
//         BatchList{
//             id:None,

//         }
//     }
// }

#[derive(Debug,Serialize,Deserialize)]
pub struct OIn{
    pub cust_id:i32,
    pub m_batches: bool,
    pub amount: f64,
    pub pending_amount: f64,
    pub order_list:Vec<OList>,
    pub batch_data:Option<Vec<BIn>>,
    pub msg: String,
    pub n_weights: i32,
    pub cases: i32,
    pub bottles: i32,
}
impl OIn{
    pub fn split(self,due_date:i64)->(Order,Vec<OList>,Option<Vec<BIn>>){
        let delta=self.amount-self.pending_amount;
        let status=if delta > 0.0{1}else{0};
        let date=Utc::now().naive_utc().into();
        (
            Order{
                order_id:None,
                cust_id:self.cust_id,
                m_batches:self.m_batches,
                amount:self.amount,
                pending_amount:self.pending_amount,
                order_date:date,
                status:status,
                msg: self.msg,
                n_weight:self.n_weights,
                cases:self.cases,
                bottles:self.bottles,
                term: (date+Duration::days(due_date)).into(),
            },
            self.order_list,
            self.batch_data
        )

    }
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = packages)]
pub struct Package {
    pub descs: String,
    pub bottel_in_pallet: i16,
    pub price: f32,
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: Option<i64>,
    pub order_id: i64,
    pub amount: f64,
    pub method: String,
    pub time: chrono::NaiveDateTime,
    pub status: i16,
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
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

#[derive(Debug,Serialize,Deserialize)]
pub struct SIn{
    pub carrier: String,
    pub tracking_no: String,
    pub eda: chrono::NaiveDate,
    pub cost: f32,
    pub n_weight: i32,
    pub address: String,
    pub batch_id: i64,
}

#[derive(Queryable,Selectable,Insertable,Debug,QueryableByName,Serialize,Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = user_watchdog)]
pub struct UserWatchdog {
    pub id: Option<i64>,
    pub username: String,
    pub date_time: chrono::NaiveDateTime,
    pub descs: String,
}

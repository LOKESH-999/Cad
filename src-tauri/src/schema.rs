// @generated automatically by Diesel CLI.

diesel::table! {
    auth (username) {
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 25]
        passwd -> Varchar,
    }
}

diesel::table! {
    batch (id) {
        id -> Nullable<Int8>,
        order_id -> Int8,
        total -> Float8,
        d_date -> Date,
        #[max_length = 255]
        msg -> Varchar,
        term -> Date,
    }
}

diesel::table! {
    batch_list (id) {
        id -> Nullable<Int8>,
        batch_id -> Int8,
        order_id -> Int8,
        #[max_length = 50]
        oil -> Varchar,
        #[max_length = 50]
        brand -> Varchar,
        n_weight -> Int4,
        cases -> Int4,
        bottles -> Int4,
        cost -> Float8,
    }
}

diesel::table! {
    brands (brand) {
        #[max_length = 50]
        brand -> Varchar,
    }
}

diesel::table! {
    customers (cust_id) {
        cust_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 15]
        hst -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 60]
        primary_email -> Varchar,
        phone -> Int8,
        date -> Timestamp,
    }
}

diesel::table! {
    e_ids (email) {
        #[max_length = 80]
        email -> Varchar,
        cust_id -> Int4,
    }
}

diesel::table! {
    order_list (id) {
        id -> Nullable<Int8>,
        order_id -> Int8,
        #[max_length = 50]
        oil -> Varchar,
        #[max_length = 50]
        brand -> Varchar,
        cases -> Int4,
        bottles -> Int4,
        cost -> Float8,
        n_weights -> Int4,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Nullable<Int8>,
        cust_id -> Int4,
        m_batches -> Bool,
        amount -> Float8,
        pending_amount -> Float8,
        order_date -> Timestamp,
        status -> Int2,
        #[max_length = 255]
        msg -> Varchar,
        n_weight -> Int4,
        cases -> Int4,
        bottles -> Int4,
        term -> Date,
    }
}

diesel::table! {
    packages (descs) {
        #[max_length = 60]
        descs -> Varchar,
        bottel_in_pallet -> Int2,
        price -> Float4,
    }
}

diesel::table! {
    payments (id) {
        id -> Nullable<Int8>,
        order_id -> Int8,
        amount -> Float8,
        #[max_length = 60]
        method -> Varchar,
        time -> Timestamp,
        status -> Int2,
    }
}

diesel::table! {
    shipping_details (ship_id) {
        ship_id -> Nullable<Int8>,
        #[max_length = 10]
        carrier -> Varchar,
        #[max_length = 60]
        tracking_no -> Varchar,
        eda -> Date,
        cost -> Float4,
        n_weight -> Int4,
        #[max_length = 255]
        address -> Varchar,
        batch_id -> Int8,
    }
}

diesel::table! {
    user_watchdog (id) {
        id -> Nullable<Int8>,
        #[max_length = 50]
        username -> Varchar,
        date_time -> Timestamp,
        #[max_length = 255]
        descs -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    batch,
    batch_list,
    brands,
    customers,
    e_ids,
    order_list,
    orders,
    packages,
    payments,
    shipping_details,
    user_watchdog,
);

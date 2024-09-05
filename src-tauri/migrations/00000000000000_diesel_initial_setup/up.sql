CREATE TABLE orders (
    order_id BIGSERIAL PRIMARY KEY,
    cust_id INTEGER NOT NULL,
    m_batches BOOLEAN NOT NULL,
    amount DOUBLE PRECISION NOT NULL CHECK (amount > 0),
    pending_amount DOUBLE PRECISION NOT NULL CHECK (pending_amount > 0),
    order_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date DATE NOT NULL,
    status SMALLINT NOT NULL CHECK (status > 0)
);
CREATE TABLE order_list (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL,
    oil VARCHAR(50) NOT NULL,
    brand VARCHAR(50) NOT NULL,
    cases INTEGER NOT NULL CHECK (cases > 0),
    bottles INTEGER NOT NULL CHECK (bottles > 0),
    cost DOUBLE PRECISION NOT NULL CHECK (cost > 0),
    n_weights INTEGER NOT NULL CHECK (n_weights > 0),
    msg VARCHAR(255) NOT NULL,
    term DATE NOT NULL
);
CREATE TABLE batch (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL,
    total DOUBLE PRECISION NOT NULL CHECK (total > 0),
    d_date DATE NOT NULL
);
CREATE TABLE batch_list (
    id BIGSERIAL PRIMARY KEY,
    batch_id BIGINT NOT NULL,
    order_id BIGINT NOT NULL,
    oil VARCHAR(50) NOT NULL,
    brand VARCHAR(50) NOT NULL,
    n_weight INTEGER NOT NULL CHECK (n_weight > 0),
    cases INTEGER NOT NULL CHECK (cases > 0),
    bottles INTEGER NOT NULL CHECK (bottles > 0),
    cost DOUBLE PRECISION NOT NULL CHECK (cost > 0),
    msg VARCHAR(255) NOT NULL,
    term DATE NOT NULL
);
CREATE TABLE shipping_details (
    ship_id BIGSERIAL PRIMARY KEY,
    carrier VARCHAR(10) NOT NULL,
    tracking_no VARCHAR(60) NOT NULL,
    eda DATE NOT NULL,
    cost FLOAT4 NOT NULL CHECK (cost > 0),
    n_weight INTEGER NOT NULL CHECK (n_weight > 0),
    address VARCHAR(255) NOT NULL,
    batch_id BIGINT UNIQUE NOT NULL
);
CREATE TABLE packages (
    descs VARCHAR(60) UNIQUE NOT NULL,
    bottel_in_pallet SMALLINT NOT NULL CHECK (bottel_in_pallet > 0),
    price REAL NOT NULL CHECK (price > 0),
    PRIMARY KEY (descs)
);
CREATE TABLE brands (
    brand VARCHAR(50) PRIMARY KEY
);
CREATE TABLE payments (
   id BIGSERIAL PRIMARY KEY,
   order_id BIGINT NOT NULL,
   amount DOUBLE PRECISION NOT NULL,
   method VARCHAR(60) NOT NULL,
   time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   status SMALLINT NOT NULL
);
CREATE TABLE user_watchdog (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    date_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    descs VARCHAR(255) NOT NULL
);
CREATE TABLE customers (
    cust_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    hst VARCHAR(15) NOT NULL,
    address VARCHAR(255) NOT NULL,
    primary_email VARCHAR(60) NOT NULL,
    phone BIGINT NOT NULL,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE e_ids (
    email VARCHAR(80) PRIMARY KEY,
    cust_id INTEGER NOT NULL
);
CREATE TABLE auth (
    username VARCHAR(50) NOT NULL PRIMARY KEY,
    passwd VARCHAR(25) NOT NULL
);
CREATE TABLE purchase (
    id BIGSERIAL PRIMARY KEY,
    account_number BIGINT NOT NULL,
    purchase_datetime TIMESTAMP NOT NULL,
    purchase_amount DECIMAL(10, 2) NOT NULL,
    post_date DATE NOT NULL,
    purchase_number INT NOT NULL,
    merchant_number VARCHAR(50) NOT NULL,
    merchant_name VARCHAR(50) NOT NULL,
    merchant_state CHAR(2) NOT NULL,
    merchant_category_code SMALLINT NOT NULL,
    FOREIGN KEY (account_number) REFERENCES account(account_number)
);
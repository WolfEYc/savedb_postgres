CREATE TABLE purchase (
    account_number BIGINT NOT NULL,
    purchase_datetime TIMESTAMP NOT NULL,
    purchase_amount DECIMAL(10, 2) NOT NULL,
    post_date DATE NOT NULL,
    purchase_number INT NOT NULL,
    merchant_number VARCHAR(50) NOT NULL,
    merchant_name VARCHAR(50) NOT NULL,
    merchant_state VARCHAR(2) NOT NULL,
    merchant_category_code SMALLINT NOT NULL,
    PRIMARY KEY (account_number, purchase_number),    
    FOREIGN KEY (account_number) REFERENCES account(account_number)
);

CREATE INDEX ON purchase ((lower(merchant_name)));
CREATE INDEX ON purchase ((lower(merchant_number)));
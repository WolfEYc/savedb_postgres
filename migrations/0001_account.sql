CREATE TABLE account (
    account_number BIGINT PRIMARY KEY,
    mobile_number BIGINT NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    ssn INT NOT NULL,
    dob DATE NOT NULL,
    zip INT NOT NULL,
    account_state VARCHAR(2) NOT NULL,
    city VARCHAR(50) NOT NULL,
    unit SMALLINT,
    street_address VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL
);
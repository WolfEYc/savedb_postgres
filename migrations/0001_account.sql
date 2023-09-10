CREATE TABLE account (
    account_number BIGINT PRIMARY KEY,
    mobile_number VARCHAR(10) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    ssn VARCHAR(11) NOT NULL,
    dob DATE NOT NULL,
    zip INT NOT NULL,
    account_state VARCHAR(2) NOT NULL,
    city VARCHAR(50) NOT NULL,
    unit SMALLINT,
    street_address VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL
);

CREATE INDEX ON account ((lower(last_name)));
CREATE INDEX ON account ((lower(first_name)));
CREATE INDEX ON account ((lower(city)));
CREATE INDEX ON account ((lower(street_address)));
CREATE INDEX ON account ((lower(email_address)));
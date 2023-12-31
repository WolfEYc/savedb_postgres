CREATE TABLE outliers (
	account_number BIGINT NOT NULL,
	purchase_number INT NOT NULL,
	PRIMARY KEY (account_number, purchase_number),
	FOREIGN KEY (account_number, purchase_number) REFERENCES purchase(account_number, purchase_number)
);
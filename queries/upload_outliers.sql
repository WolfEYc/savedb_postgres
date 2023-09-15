INSERT INTO outliers (
	account_number,
	purchase_number
) SELECT * FROM UNNEST (
    $1::BIGINT[],
    $2::INT[]
) ON CONFLICT (account_number, purchase_number)
DO NOTHING
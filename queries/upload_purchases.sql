INSERT INTO purchase (
    account_number,
    purchase_datetime,
    purchase_amount,
    post_date,
    purchase_number,
    merchant_number,
    merchant_name,
    merchant_state,
    merchant_category_code
) SELECT * FROM UNNEST (
    $1::BIGINT[],
    $2::TIMESTAMP[],
    $3::DECIMAL(10,2)[],
    $4::DATE[],
    $5::INT[],
    $6::VARCHAR[],
    $7::VARCHAR[],
    $8::VARCHAR[],
    $9::SMALLINT[]
)
ON CONFLICT (account_number, purchase_number)
DO UPDATE SET
    purchase_datetime = EXCLUDED.purchase_datetime,
    purchase_amount = EXCLUDED.purchase_amount,
    post_date = EXCLUDED.post_date,
    merchant_number = EXCLUDED.merchant_number,
    merchant_name = EXCLUDED.merchant_name,
    merchant_state = EXCLUDED.merchant_state,
    merchant_category_code = EXCLUDED.merchant_category_code
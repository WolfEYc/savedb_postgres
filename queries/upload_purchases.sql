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
    $3::DECIMAL[],
    $4::DATE[],
    $5::INT[],
    $6::VARCHAR[],
    $7::VARCHAR[],
    $8::CHAR[],
    $9::SMALLINT[]
)
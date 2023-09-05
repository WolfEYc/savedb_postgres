INSERT INTO account (
    account_number,
    mobile_number,
    email_address,
    ssn,
    dob,
    zip,
    account_state,
    city,
    unit,
    street_address,
    first_name,
    last_name
) SELECT * FROM UNNEST (
    $1::BIGINT[],
    $2::BIGINT[],
    $3::VARCHAR[],
    $4::INT[],
    $5::DATE[],
    $6::INT[],
    $7::CHAR[],
    $8::VARCHAR[],
    $9::SMALLINT[],
    $10::VARCHAR[],
    $11::VARCHAR[],
    $12::VARCHAR[]
)
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
    $7::VARCHAR[],
    $8::VARCHAR[],
    $9::SMALLINT[],
    $10::VARCHAR[],
    $11::VARCHAR[],
    $12::VARCHAR[]
)
ON CONFLICT (account_number)
DO UPDATE SET
    mobile_number = EXCLUDED.mobile_number,
    email_address = EXCLUDED.email_address,
    ssn = EXCLUDED.ssn,
    dob = EXCLUDED.dob,
    zip = EXCLUDED.zip,
    account_state = EXCLUDED.account_state,
    city = EXCLUDED.city,
    unit = EXCLUDED.unit,
    street_address = EXCLUDED.street_address,
    first_name = EXCLUDED.first_name,
    last_name = EXCLUDED.last_name
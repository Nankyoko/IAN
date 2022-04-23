ALTER TABLE users 
    ADD CONSTRAINT user_unique UNIQUE(username);

INSERT INTO users (first_name, last_name, username)
    VALUES ('IAN', 'System', 'System'),
    ('Headmaster', 'System', 'Headmaster');

CREATE FUNCTION get_system_uuid()
RETURNS UUID AS $IAN_id$
DECLARE 
    IAN_id uuid;
BEGIN
    SELECT id INTO IAN_id FROM users WHERE username = 'System';
    RETURN IAN_id;
END; $IAN_id$ LANGUAGE plpgsql;

CREATE FUNCTION get_headmaster_uuid()
RETURNS UUID AS $HEAD_id$
DECLARE 
    HEAD_id uuid;
BEGIN
    SELECT id INTO HEAD_id FROM users WHERE username = 'Headmaster';
    RETURN HEAD_id;
END; $HEAD_id$ LANGUAGE plpgsql;

CREATE TABLE ledger_accounts( 
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL DEFAULT get_headmaster_uuid(),
    account_name VARCHAR(32) NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(), 

    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE SET DEFAULT
);

CREATE TABLE journal_headers(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_amount MONEY NOT NULL DEFAULT 0,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_date TIMESTAMP NOT NULL DEFAULT NOW(), 
    memo TEXT,
    cleared BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE journal_lines(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    header_id UUID NOT NULL,
    account_id UUID NOT NULL,
    debit MONEY NOT NULL DEFAULT 0,
    credit MONEY NOT NULL DEFAULT 0,

    CONSTRAINT fk_header
        FOREIGN KEY(header_id)
            REFERENCES journal_headers(id),

    CONSTRAINT fk_account
        FOREIGN KEY(account_id)
            REFERENCES ledger_accounts(id)
);

CREATE TABLE gl_entries( 
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_header_id UUID NOT NULL,
    ledger_account_id UUID NOT NULL,
    debit MONEY NOT NULL DEFAULT 0,
    credit MONEY NOT NULL DEFAULT 0,
    created_date TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_date TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_header
        FOREIGN KEY(journal_header_id)
            REFERENCES journal_headers(id),

    CONSTRAINT fk_account
        FOREIGN KEY(ledger_account_id)
            REFERENCES ledger_accounts(id)
);

ALTER TABLE secure_user_info 
    ADD CONSTRAINT fk_users FOREIGN KEY(user_id) REFERENCES users(id);
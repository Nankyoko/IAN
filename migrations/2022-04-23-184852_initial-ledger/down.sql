-- This file should undo anything in `up.sql`
ALTER TABLE secure_user_info DROP CONSTRAINT fk_users;
DROP TABLE gl_entries;
DROP TABLE journal_lines;
DROP TABLE journal_headers;
DROP TABLE ledger_accounts;
DROP FUNCTION get_headmaster_uuid;
DROP FUNCTION get_system_uuid;
DELETE FROM users WHERE username IN ('System', 'Headmaster');
ALTER TABLE users DROP CONSTRAINT user_unique;
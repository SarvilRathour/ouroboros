-- Add migration script here
CREATE TYPE roles as ENUM {'Admin','James_Bond'}
CREATE TABLE users{
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    
}

-- Your SQL goes here
CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    item_name VARCHAR(250) NOT NULL,
    current_stock INTEGER NOT NULL DEFAULT 0
)

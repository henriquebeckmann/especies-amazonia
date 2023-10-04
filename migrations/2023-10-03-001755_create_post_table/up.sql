-- Your SQL goes here
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    state_name VARCHAR(255) NOT NULL,
    state_abbreviation CHAR(2) NOT NULL,
    city VARCHAR(255) NOT NULL,
    UNIQUE(state_abbreviation, city)
);

CREATE TABLE posts (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    image_url VARCHAR NOT NULL,
    date_picture TIMESTAMP,
    description VARCHAR(255),
    family VARCHAR(100),
    gender VARCHAR(100),
    specie VARCHAR(100),
    location INT NOT NULL,
    locality VARCHAR(100) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

ALTER TABLE posts ADD FOREIGN KEY (location) REFERENCES locations(id);

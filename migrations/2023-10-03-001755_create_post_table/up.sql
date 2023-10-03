-- Your SQL goes here
CREATE TABLE location (
    id SERIAL PRIMARY KEY,
    state VARCHAR(100) NOT NULL,
    acronym CHAR(2) NOT NULL,
    county VARCHAR(100) NOT NULL
);

CREATE TABLE post (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    imageUrl VARCHAR NOT NULL,
    datePicture DATE,
    description VARCHAR(255),
    family VARCHAR(100),
    gender VARCHAR(100),
    specie VARCHAR(100),
    location INT NOT NULL,
    locality VARCHAR(100) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    publishedAt TIMESTAMP NOT NULL
);

ALTER TABLE Post ADD FOREIGN KEY (location) REFERENCES Location(id);

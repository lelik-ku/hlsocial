CREATE EXTENSION citext;

CREATE DOMAIN email AS citext
   CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE IF NOT EXISTS users(
   user_id BIGSERIAL NOT NULL PRIMARY KEY,
   first_name VARCHAR (50) NOT NULL,
   second_name VARCHAR (50),
   email email UNIQUE NOT NULL,
   pwhash VARCHAR (150) NOT NULL,
   gender CHAR (1),
   birthdate DATE,
   biography VARCHAR (300),
   city VARCHAR (50),
   CHECK (gender='m' OR gender='f' OR gender=NULL )
);

CREATE INDEX ON users (email);

INSERT INTO users (first_name, second_name, pwhash, email, gender, birthdate, biography, city)
VALUES ('admin', 'admin', '$6$7Cdj6ggt3a33VYgV$vCjSZQ7Bu0s4fcKj/HnUpOaI6h.3vz671m6jHo9RIOtR4jCj/lJkviva6QvAn6HemOC5Qyt9y942FowLQKFto0', 'admin@admin.admin', 'f', NULL, 'Administrator', NULL),
       ('admin1', 'admin1', '$6$7Cdj6ggt3a33VYgV$vCjSZQ7Bu0s4fcKj/HnUpOaI6h.3vz671m6jHo9RIOtR4jCj/lJkviva6QvAn6HemOC5Qyt9y942FowLQKFto0', 'admin1@admin.admin', 'm', NULL, 'Administrator1', NULL) 
ON CONFLICT (email) DO NOTHING;

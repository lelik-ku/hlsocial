CREATE EXTENSION citext;

CREATE DOMAIN email AS citext
   CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE IF NOT EXISTS users(
   user_id BIGSERIAL NOT NULL PRIMARY KEY,
   first_name VARCHAR (50) NOT NULL,
   second_name VARCHAR (50),
   email email UNIQUE NOT NULL,
   pwhash VARCHAR (100) NOT NULL,
   gender CHAR (1),
   birthdate DATE,
   biography VARCHAR (300),
   city VARCHAR (50),
   CHECK (gender='m' OR gender='f' OR gender=NULL )
);

CREATE INDEX ON users (email);

INSERT INTO users (first_name, second_name, pwhash, email, gender, birthdate, biography, city)
VALUES ('admin', 'admin', '$2b$12$xH2IFRor9oLJPAHoFG/5jeZfc9MR1S5RTohgaNdN1wvaWiAHt1QvW', 'admin@admin.admin', 'f', NULL, 'Administrator', NULL),
       ('admin1', 'admin1', 'test1', 'admin1@admin.admin', 'm', NULL, 'Administrator1', NULL) 
ON CONFLICT (email) DO NOTHING;

CREATE TABLE IF NOT EXISTS users(
   user_id BIGSERIAL NOT NULL PRIMARY KEY,
   first_name VARCHAR (50),
   second_name VARCHAR (50),
   pwhash VARCHAR (100) NOT NULL,
   email VARCHAR (150) UNIQUE NOT NULL,
   gender CHAR (1),
   birthdate DATE,
   biography VARCHAR (300),
   city VARCHAR (50),
   CHECK (gender='m' OR gender='f')
);

INSERT INTO users (first_name, second_name, pwhash, email, gender, birthdate, biography, city)
VALUES ('admin', 'admin', 'test', 'admin@admin.admin', 'f', NULL, 'Administrator', NULL),
       ('admin1', 'admin1', 'test1', 'admin1@admin.admin', 'm', NULL, 'Administrator1', NULL) 
ON CONFLICT (email) DO NOTHING;

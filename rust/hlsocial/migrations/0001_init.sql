CREATE TABLE IF NOT EXISTS users(
   user_id BIGSERIAL NOT NULL PRIMARY KEY,
   first_name VARCHAR (50),
   second_name VARCHAR (50),
   pwhash VARCHAR (100) NOT NULL,
   email VARCHAR (150) UNIQUE NOT NULL,
   gender VARCHAR (6),
   birthdate DATE,
   biography VARCHAR (300),
   city VARCHAR (50)
);

INSERT INTO users (first_name, second_name, pwhash, email, gender, birthdate, biography, city)
VALUES ('admin', 'admin', 'test', 'admin@admin.admin', NULL, NULL, 'Administrator', NULL) 
ON CONFLICT (email) DO NOTHING;

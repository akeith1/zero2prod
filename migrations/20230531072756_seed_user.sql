-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
'b0ff052d-b053-4dc2-b93a-196e1eb4807f',
'admin',
 '$argon2id$v=19$m=15000,t=2,p=1$6yRc6Yt4We/C+3e4JAb8Mg$cu98zwtSpDHBD19tioWL/6cX5382s3DARzCpzt5n+gc'

);

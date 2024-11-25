CREATE TABLE "user" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "username" varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL,
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "status" tinyint(1) DEFAULT 1
);
CREATE TABLE "user_apps" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "uid" integer NOT NULL,
  "title" varchar(255) NOT NULL,
  "secret_id" varchar(255) NOT NULL,
  "secret_key" varchar(255) NOT NULL,
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "status" tinyint(1) DEFAULT 1
);

CREATE TABLE "user_domain" (
  "id" integer PRIMARY KEY AUTOINCREMENT,
  "appid" integer NOT NULL,
  "host" varchar(255) NOT NULL,
  "domain" varchar(255) NOT NULL,
  "ip_type" varchar(255) DEFAULT 1,
  "ip" varchar(255) NOT NULL,
  "record_id" integer NOT NULL,
  "weight" integer DEFAULT 1,
  "ttl" integer DEFAULT 600
  "created_at" datetime DEFAULT CURRENT_TIMESTAMP,
  "updated_at" datetime DEFAULT CURRENT_TIMESTAMP,
);
-- Add up migration script here
create table setting (
    "key" varchar(255) primary key,
    "value" varchar(255) ,
    description varchar(255)
);
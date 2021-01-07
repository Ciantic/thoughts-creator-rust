
CREATE TABLE articles (
  id                   VARCHAR (128)   NOT NULL PRIMARY KEY,
  hash                 VARCHAR (64)    NOT NULL,
  published            DATETIME        NOT NULL,
  modified             DATETIME        NOT NULL,
  modified_on_disk     DATETIME        NOT NULL,
  local_path           VARCHAR (2048)  NOT NULL UNIQUE,
  server_path          VARCHAR (2048)  NOT NULL UNIQUE,
  title                VARCHAR (2048)  NOT NULL DEFAULT "",
  html                 VARCHAR (10048) NOT NULL DEFAULT ""
);

CREATE TABLE resources (
  id                   VARCHAR (128)   NOT NULL PRIMARY KEY,
  modified_on_disk     DATETIME        NOT NULL,
  local_path           VARCHAR (2048)  NOT NULL UNIQUE,
  server_path          VARCHAR (2048)  NOT NULL UNIQUE
);

CREATE TABLE images (
  id                   VARCHAR (128)   NOT NULL PRIMARY KEY,
  modified_on_disk   DATETIME        NOT NULL,
  width              INTEGER         NOT NULL,
  height             INTEGER         NOT NULL,
  local_path         VARCHAR (2048)  NOT NULL UNIQUE,
  server_path        VARCHAR (2048)  NOT NULL UNIQUE
);

CREATE TABLE urls (
  id                   VARCHAR (128)   NOT NULL PRIMARY KEY,
  url                VARCHAR (1024)  NOT NULL
);
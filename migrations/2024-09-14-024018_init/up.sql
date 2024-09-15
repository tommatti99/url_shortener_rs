CREATE TABLE db_links (
    "dt" DATE NOT NULL,
    "original_link" VARCHAR NOT NULL, PRIMARY KEY (original_link),
    "short_link" VARCHAR NOT NULL,
    "clicks" INTEGER NOT NULL
);
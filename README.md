# rscoDB

![Build](https://github.com/Jordan-M-Young/rscoDB/actions/workflows/build.yml/badge.svg?event=push) ![Tests](https://github.com/Jordan-M-Young/rscoDB/actions/workflows/test.yml/badge.svg?event=push)

This is a from scratch implementation of a Relational Database in rust. The goal here is for myself and perhaps others 
to more deeply understand the inner workings of popular relational database systems like postgres by attempting to build
an analog system. Some things this project will focus on are:

    - SQL Syntax
        - Supporting Basic Statements/Logic (SELECT, INSERT, CREATE, FROM, etc...)
        - Validating command syntax
    - Command Plans
        - SQL syntax -> Plan structs containing information required to programmatically execute command
        - Plan validation 
    - DataStorage
        - In memory -> Single File (Sqlite) -> PG_DATA directory (Postgres)
    - ACID
    - External Client
        - Support external client access
        - Write basic python client

## Current Focus

Currently I'm focusing on finishing up sine basic SQL syntax and accompanying logic. You can see what SQL statements are supported currently. At this time input commmands are very lightly validated, so further work will be required for a more robust validation system.


## Run 

```bash
cargo run
```

# SQL

For rscoDB, we'll try to mirror the SQL syntax used by [Postgres](https://www.postgresql.org/docs/current/sql-syntax.html).

## Supported Commands

### Database

#### Create Database

```sql
CREATE DATABASE my_database
```
#### Connect to Database

```sql
CONNECT database_name
```

#### Show Databases

```sql
SHOW DATABASE
```
#### Drop Database

```sql
DROP DATABASE db_name
```
### Table

#### Create Table

```sql
CREATE TABLE table_name (field_a int, field_b varchar)
```

#### Show Database Tables
```sql
SHOW TABLE
```

#### Drop Database Table
```sql
DROP TABLE table_name
```

### Inserting Rows

```sql
INSERT INTO table_name VALUES (1, hello, 3.0)
```

### Querying

#### The only query right now lol

```sql
SELECT * FROM table_name
```



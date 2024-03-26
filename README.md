# rscoDB
Relational Database Implementation

## Run 

```bash
cargo run
```




# SQL

For rscoDB, we'll try to mirror the SQL syntax used by [Postgres](https://www.postgresql.org/docs/current/sql-syntax.html).

## Supported Commands

### Create Database

```sql
CREATE DATABASE my_database
```
### Show Databases

```sql
SHOW DATABASE
```
### Drop Database

```sql
DROP DATABASE db_name
```

### Create Table

```sql
CREATE TABLE table_name (field_a int, field_b varchar)
```

### Show Database Tables
```sql
SHOW TABLE
```

### Drop Database Table
```sql
DROP TABLE table_name
```



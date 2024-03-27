# rscoDB
Relational Database Implementation

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



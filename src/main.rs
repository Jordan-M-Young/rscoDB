use std::{any::Any, collections::HashMap, io::prelude::*};

use crate::{
    command::{Statement, StatementType},
    plan::MetaPlan,
    table::{DataBase, DataTypes, Table},
};
pub mod command;
pub mod execute;
pub mod parse;
pub mod plan;
pub mod table;

fn main() {
    println!("Welcome to roscoeDB v0.0.0");

    let mut databases: HashMap<String, DataBase> = HashMap::new();
    let mut default_tables: HashMap<String, Table> = HashMap::new();
    let mut current_db = DataBase {
        tables: default_tables,
        name: "default".to_string(),
    };

    databases.insert("default".to_string(), current_db);
    let current_db_name = "default".to_string();
    let mut manifest = table::Manifest { databases };

    loop {
        print!("rscoDB > ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        //tokenize command into statements
        let statements: Vec<&str> = buf.split(' ').collect();

        // match text statements into specific variants of StatementType enum
        let mut parsed_statements: Vec<StatementType> = vec![];
        for statement in statements {
            match command::match_statment(statement) {
                StatementType::NullType => {}
                non_null_type => parsed_statements.push(non_null_type),
            }
        }

        let plan = plan::build_plan(parsed_statements);

        match plan {
            plan::PlanTypes::MetaType(plan) => {
                if plan.command == ".quit".to_string() {
                    println!("Exiting rscoDB");
                    break;
                }

                execute::execute_meta_plan(plan)
            }
            plan::PlanTypes::SelectType(plan) => {
                if plan.columns[0] == "*" {
                    match manifest.databases.get(&current_db_name) {
                        Some(database) => match database.tables.get(&plan.table) {
                            Some(table) => {
                                for row in &table.rows {
                                    println!("{:?}", row)
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            plan::PlanTypes::InsertPlan(plan) => {
                match manifest.databases.get(&current_db_name) {
                    Some(database) => {
                        match database.tables.get(&plan.table) {
                            Some(table) => {
                                // pub struct Table {
                                //     pub name: String,
                                //     pub rows: Vec<Vec<DataTypes>>,
                                //     pub n_columns: usize,
                                //     pub columns: HashMap<String, DataTypes>,
                                // }
                                let mut table = table.clone();
                                let mut database = database.clone();
                                table.rows.push(plan.values);

                                database.tables.insert(plan.table, table);
                                manifest.databases.insert(current_db_name.clone(), database);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            plan::PlanTypes::CreateTableType(plan) => {
                let n_columns = plan.schema.len();
                let mut columns: HashMap<String, DataTypes> = HashMap::new();
                for schema in plan.schema {
                    columns.insert(schema.column_name, schema.column_type);
                }

                let rows: Vec<Vec<DataTypes>> = vec![];
                let table = Table {
                    name: plan.table,
                    rows,
                    n_columns,
                    columns,
                };

                match manifest.databases.get(&current_db_name) {
                    Some(db) => {
                        let mut db = db.clone();
                        db.tables.insert(table.name.clone(), table);
                        manifest.databases.insert(current_db_name.clone(), db);
                    }
                    _ => {}
                };
            }
            plan::PlanTypes::CreateDataBaseType(plan) => {
                let tables: HashMap<String, Table> = HashMap::new();
                let table_name = plan.db_name.clone();
                manifest.databases.insert(
                    plan.db_name,
                    DataBase {
                        tables,
                        name: table_name,
                    },
                );
            }
            plan::PlanTypes::ShowDataBasesPlan => execute::execute_show_plan(&manifest),
            plan::PlanTypes::ShowTablesPlan => {
                let current_db = &manifest.databases.get(&current_db_name).unwrap();
                execute::execute_show_table_plan(&current_db)
            }
            plan::PlanTypes::DropTablePlan(table_name) => {
                match manifest.databases.get(&current_db_name) {
                    Some(entry) => {
                        let mut entry = entry.clone();
                        entry.tables.remove(&table_name);
                        manifest.databases.insert(current_db_name.clone(), entry);
                    }

                    _ => {}
                }
            }
            plan::PlanTypes::DropDataBasePlan(db_name) => {
                manifest.databases.remove(&db_name);
            }
            _ => {
                println!("Not implemented yet!")
            }
        }
    }
}

// let  alpha_numeric = "1234567890";
// let mut  x = "123".to_string();
// println!("{}",x);

// let mut  y = "1.0".to_string();
// let mut z = "one".to_string();

// let a = parse::parse_string(&y);
// let b = parse::parse_string(&x);
// let c = parse::parse_string(&z);
// println!("{:?}",a);
// println!("{:?}",b);
// println!("{:?}",c);

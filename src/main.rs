use std::{collections::HashMap, io::prelude::*};

use crate::{
    command::{Statement, StatementType},
    plan::MetaPlan,
    table::{DataBase, DataTypes, Table},
};
pub mod command;
pub mod execute;
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
            plan::PlanTypes::SelectType(plan) => execute::execute_select_plan(plan),
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
            _ => {
                println!("Not implemented yet!")
            }
        }

        // if buf.starts_with(".") {
        //     match command::process_meta_command(&buf) {
        //         command::CommandResult::CommandSuccess => {
        //             println!("Executed: {}", buf)
        //         }
        //         command::CommandResult::CommandFailure => {
        //             println!("Unrecognized command: {}", buf)
        //         }
        //         command::CommandResult::CommandQuit => {
        //             println!("Exiting...");
        //             break;
        //         }
        //     }
        // } else {
        //     match command::process_command(&buf) {
        //         command::CommandResult::CommandSuccess => {
        //             println!("Executed: {}", buf)
        //         }
        //         command::CommandResult::CommandFailure => {
        //             println!("Unrecognized command: {}", buf)
        //         }
        //         _ => {}
        //     }
        // }
    }
}

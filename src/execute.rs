use crate::{
    plan::{MetaPlan, SelectPlan},
    table::{DataBase, Manifest},
};

pub fn execute_meta_plan(meta_plan: MetaPlan) {
    println!("Executing {:?}", meta_plan.command)
}

pub fn execute_select_plan(select_plan: SelectPlan) {
    println!("Executing {:?}", select_plan)
}

pub fn execute_show_plan(manifest: &Manifest) {
    println!("Database\n--------");
    for (name, _database) in &manifest.databases {
        println!("{}", name)
    }
}

pub fn execute_show_table_plan(db: &DataBase) {
    println!("Database: {}\n--------", db.name);
    for (name, _table) in &db.tables {
        println!("{}", name)
    }
    println!("-------")
}

use crate::{
    plan::{MetaPlan, SelectPlan},
    table::Manifest,
};

pub fn execute_meta_plan(meta_plan: MetaPlan) {
    println!("Executing {:?}", meta_plan.command)
}

pub fn execute_select_plan(select_plan: SelectPlan) {
    println!("Executing {:?}", select_plan)
}

pub fn execute_show_plan(manifest: &Manifest) {
    println!("Database\n--------");
    for (name, database) in &manifest.databases {
        println!("{}", name)
    }
}

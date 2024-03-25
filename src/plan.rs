use std::collections::HashMap;

use crate::{command::StatementType, table::ColumnSchema};

#[derive(Debug)]
pub struct SelectPlan {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Debug)]
pub struct MetaPlan {
    pub command: String,
}

#[derive(Debug)]
pub struct CreateDataBasePlan {
    pub db_name: String,
}

#[derive(Debug)]
pub struct CreateTablePlan {
    pub table: String,
    pub schema: Vec<ColumnSchema>,
}

#[derive(Debug)]
pub enum PlanTypes {
    SelectType(SelectPlan),
    InsertPlan,
    CreateDataBaseType(CreateDataBasePlan),
    CreateTableType(CreateTablePlan),
    MetaType(MetaPlan),
    InvalidPlan,
    ShowPlan,
}

#[derive(Debug)]
pub enum PlanState {
    Valid,
    Invalid,
}

pub fn build_plan(statements: Vec<StatementType>) -> PlanTypes {
    let mut ptr_0 = 0;

    let plan_type = match &statements[ptr_0] {
        StatementType::SelectType => {
            let plan = build_select_plan(&statements);
            println!("{:?}", plan);
            match validate_select_plan(&plan) {
                PlanState::Valid => PlanTypes::SelectType(plan),
                PlanState::Invalid => PlanTypes::InvalidPlan,
            }
        }
        StatementType::InsertType => PlanTypes::InsertPlan,
        StatementType::MetaType(value) => PlanTypes::MetaType(MetaPlan {
            command: value.to_string(),
        }),
        StatementType::CreateType => {
            let ptr_1 = 1;
            match &statements[ptr_1] {
                StatementType::DataBaseType => {
                    if statements.len() == 3 {
                        let plan = build_create_db_plan(&statements);
                        return PlanTypes::CreateDataBaseType(plan);
                    }
                    PlanTypes::InvalidPlan
                }
                _ => PlanTypes::InvalidPlan,
            }
        }
        StatementType::ShowType => PlanTypes::ShowPlan,
        _ => PlanTypes::InvalidPlan,
    };

    plan_type
}

fn build_create_db_plan(statements: &Vec<StatementType>) -> CreateDataBasePlan {
    let ptr_2 = 2;

    match &statements[ptr_2] {
        StatementType::NameType(db_name) => CreateDataBasePlan {
            db_name: db_name.to_string(),
        },
        _ => CreateDataBasePlan {
            db_name: "".to_string(),
        },
    }
}

fn build_select_plan(statements: &Vec<StatementType>) -> SelectPlan {
    let mut ptr_1 = 1;
    let mut column_flag = true;
    let l = statements.len();

    let mut columns: Vec<String> = vec![];
    let mut table: String = String::new();

    while ptr_1 <= l - 1 {
        match &statements[ptr_1] {
            StatementType::NameType(val) => {
                if column_flag {
                    columns.push(val.to_string());
                } else {
                    table = val.to_string()
                }
                ptr_1 += 1;
            }
            StatementType::FromType => {
                column_flag = false;
                ptr_1 += 1;
            }
            _ => {
                ptr_1 += 1;
            }
        }
    }

    SelectPlan { columns, table }
}

fn validate_select_plan(plan: &SelectPlan) -> PlanState {
    if !plan.columns.is_empty() && !plan.table.is_empty() {
        return PlanState::Valid;
    }

    PlanState::Invalid
}

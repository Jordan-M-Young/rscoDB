use crate::{
    command::{data_type_check, StatementType},
    parse::parse_string,
    table::{match_data_type, ColumnSchema, DataTypes},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SelectPlan {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Debug)]
pub struct InsertPlan {
    pub table: String,
    pub values: Vec<DataTypes>,
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
    InsertPlan(InsertPlan),
    CreateDataBaseType(CreateDataBasePlan),
    CreateTableType(CreateTablePlan),
    MetaType(MetaPlan),
    InvalidPlan,
    ShowDataBasesPlan,
    ShowTablesPlan,
    DropDataBasePlan(String),
    DropTablePlan(String),
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
        StatementType::InsertType => create_insert_plan(&statements),
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
                StatementType::TableType => {
                    let plan = build_create_table_plan(&statements);
                    return PlanTypes::CreateTableType(plan);
                }
                _ => PlanTypes::InvalidPlan,
            }
        }
        StatementType::ShowType => {
            let ptr_1 = 1;
            match &statements[ptr_1] {
                StatementType::TableType => PlanTypes::ShowTablesPlan,
                StatementType::DataBaseType => PlanTypes::ShowDataBasesPlan,
                _ => PlanTypes::InvalidPlan,
            }
        }
        StatementType::DropType => {
            let x = build_drop_plan(&statements);
            println!("{:?}", &x);
            x
        }
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

fn build_create_table_plan(statements: &Vec<StatementType>) -> CreateTablePlan {
    let mut ptr_2 = 2;
    let length = statements.len();

    let name = match &statements[ptr_2] {
        StatementType::NameType(db_name) => db_name.to_owned(),
        _ => "".to_string(),
    };

    let mut schema: Vec<ColumnSchema> = vec![];
    ptr_2 += 1;
    let mut current_col_name = String::new();
    let mut current_val_type = DataTypes::Null;

    while ptr_2 <= length - 1 {
        match &statements[ptr_2] {
            StatementType::NameType(value) => {
                current_col_name = value.to_string();
                ptr_2 += 1;
            }
            StatementType::ValueTypeType(value) => {
                current_val_type = match_data_type(&value);
                schema.push(ColumnSchema {
                    column_name: current_col_name.clone(),
                    column_type: current_val_type,
                });

                ptr_2 += 1;
            }
            _ => {}
        }
    }

    CreateTablePlan {
        table: name,
        schema,
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
                    table = val.to_string();
                    break;
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

fn build_drop_plan(statements: &Vec<StatementType>) -> PlanTypes {
    let ptr_1 = 1;
    let ptr_2 = 2;
    let name = match &statements[ptr_2] {
        StatementType::NameType(value) => value,
        _ => return PlanTypes::InvalidPlan,
    };

    match &statements[ptr_1] {
        StatementType::DataBaseType => PlanTypes::DropDataBasePlan(name.to_string()),
        StatementType::TableType => PlanTypes::DropTablePlan(name.to_string()),
        _ => return PlanTypes::InvalidPlan,
    }
}

fn create_insert_plan(statements: &Vec<StatementType>) -> PlanTypes {
    let ptr_1 = 1;
    let ptr_2 = 2;
    let ptr_3 = 3;
    let mut ptr_4 = 4;

    println!("{:?}", &statements);

    let statements_length = statements.len();
    match &statements[ptr_1] {
        StatementType::IntoType => {}
        _ => {
            return PlanTypes::InvalidPlan;
        }
    }

    let table_name = match &statements[ptr_2] {
        StatementType::NameType(name) => name,
        _ => return PlanTypes::InvalidPlan,
    };

    match &statements[ptr_3] {
        StatementType::ValuesType => {}
        _ => return PlanTypes::InvalidPlan,
    }

    let mut values: Vec<DataTypes> = vec![];
    while ptr_4 <= statements_length - 1 {
        match &statements[ptr_4] {
            StatementType::NameType(value) => {
                let value = value.replace(",", "").replace("(", "").replace(")", "");
                match parse_string(&value) {
                    Some(value) => values.push(value),
                    _ => {}
                }

                ptr_4 += 1
            }
            _ => ptr_4 += 1,
        }
    }

    let plan = InsertPlan {
        table: table_name.to_string(),
        values,
    };

    PlanTypes::InsertPlan(plan)
}

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub emp_no: i32,
    pub birth_date: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub hire_date: NaiveDate,
}
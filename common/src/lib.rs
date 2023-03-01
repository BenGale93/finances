#![warn(clippy::all, clippy::nursery)]
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Transaction {
    pub id: i64,
    pub account: String,
    pub date: NaiveDateTime,
    pub description: String,
    pub amount: f64,
    pub l1_tag: String,
    pub l2_tag: String,
    pub l3_tag: String,
}

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountSummary {
    pub name: String,
    pub amount: f64,
}

impl PartialEq for AccountSummary {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for AccountSummary {}

impl PartialOrd for AccountSummary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for AccountSummary {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConfigOptions {
    Budget(f64),
    AccountList(Vec<String>),
    PeriodItems(Vec<String>),
    BudgetItems(Vec<String>),
    Tags(Tags),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tags(HashMap<String, HashMap<String, Vec<String>>>);

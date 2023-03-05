#![warn(clippy::all, clippy::nursery)]
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
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

impl Eq for Transaction {}

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
        self.name == other.name && self.amount == other.amount
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
pub struct Config {
    budget: f64,
    account_list: Vec<String>,
    period_items: Vec<String>,
    budget_items: Vec<String>,
    tags: Tags,
}

impl Config {
    pub const fn budget(&self) -> f64 {
        self.budget
    }

    pub fn account_list(&self) -> &[String] {
        self.account_list.as_ref()
    }

    pub fn period_items(&self) -> &[String] {
        self.period_items.as_ref()
    }

    pub fn budget_items(&self) -> &[String] {
        self.budget_items.as_ref()
    }

    pub const fn tags(&self) -> &Tags {
        &self.tags
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConfigOptions {
    All(Config),
    Budget(f64),
    AccountList(Vec<String>),
    PeriodItems(Vec<String>),
    BudgetItems(Vec<String>),
    Tags(Tags),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tags(pub HashMap<String, HashMap<String, Vec<String>>>);

impl Tags {
    pub fn verify_tags(&self, l1_tag: &str, l2_tag: &str, l3_tag: &str) -> bool {
        let Some(level_2) = self.0.get(l1_tag) else {
            return false;
        };
        let Some(level_3) = level_2.get(l2_tag) else {
            return false;
        };
        level_3.contains(&l3_tag.to_owned())
    }
}

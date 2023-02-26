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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tags(HashMap<String, HashMap<String, Vec<String>>>);

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

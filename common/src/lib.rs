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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct BalanceByTime {
    pub date: String,
    pub incoming: f64,
    pub outgoing: f64,
    pub balance: f64,
}

pub type BalancesByDay = [BalanceByTime];

pub trait BalancesByDayExt {
    fn vectors(&self) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>);
    fn cumsum(&self) -> BalanceOverTime;

    fn rolling_average_cumsum(&self, window: usize) -> Option<BalanceOverTime>;
}

#[derive(PartialEq)]
pub struct BalanceOverTime {
    pub dates: Vec<String>,
    pub balances: Vec<f64>,
}

impl BalancesByDayExt for BalancesByDay {
    fn vectors(&self) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let len = self.len();
        let mut dates = Vec::with_capacity(len);
        let mut incoming = Vec::with_capacity(len);
        let mut outgoing = Vec::with_capacity(len);
        let mut balance = Vec::with_capacity(len);

        (0..len).for_each(|i| {
            dates.push(self[i].date.clone());
            incoming.push(self[i].incoming);
            outgoing.push(self[i].outgoing);
            balance.push(self[i].balance);
        });

        (dates, incoming, outgoing, balance)
    }
    fn cumsum(&self) -> BalanceOverTime {
        let len = self.len();
        let mut date = Vec::with_capacity(len);
        let mut total = Vec::with_capacity(len);

        for point in self {
            date.push(point.date.clone());
            total.push(point.balance);
        }

        total.iter_mut().fold(0.0, |acc, x| {
            *x += acc;
            *x
        });

        BalanceOverTime {
            dates: date,
            balances: total,
        }
    }

    fn rolling_average_cumsum(&self, window: usize) -> Option<BalanceOverTime> {
        let BalanceOverTime {
            dates: full_dates,
            balances: cum_bal,
        } = self.cumsum();
        let length = self.len();
        let window_index = window - 1;

        if length <= window_index {
            return None;
        }

        let mut rolling_averages: Vec<f64> = Vec::with_capacity(length);
        let dates = full_dates[window_index..length].to_owned();

        let mut sum = 0.0;

        (0..=window_index).for_each(|x| {
            let balance = &cum_bal[x];
            sum += balance;
        });

        let first_moving_average_day = sum / window as f64;
        rolling_averages.push(first_moving_average_day);

        if length == window {
            return Some(BalanceOverTime {
                dates,
                balances: rolling_averages,
            });
        }

        let mut head_balance_index: usize = 0;
        let mut tail_balance_index: usize = window;

        while tail_balance_index != length {
            let prev_rolling_average = &rolling_averages[head_balance_index];
            let head_balance = cum_bal[head_balance_index] / window as f64;
            let tail_balance = cum_bal[tail_balance_index] / window as f64;
            let current_rolling_average = prev_rolling_average - head_balance + tail_balance;
            rolling_averages.push(current_rolling_average);

            head_balance_index += 1;
            tail_balance_index += 1;
        }

        Some(BalanceOverTime {
            dates,
            balances: rolling_averages,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DateGrouping {
    Day,
    Month,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceTimeOptions {
    pub grouping: Option<DateGrouping>,
}

use std::sync::Arc;

use chrono::{Datelike, NaiveDate, Utc};
use common::{
    BudgetProgress, BudgetProgressOptions, CategorySpend, CategorySpendOptions, ConfigOptions,
};
use plotly::{
    color::NamedColor,
    common::DashType,
    layout::{Shape, ShapeLine, ShapeType},
    Bar, Layout, Plot,
};
use yew::prelude::*;
use yew_plotly::Plotly;

use crate::{api, components::CategorySpendComponent};

pub enum BudgetMsg {
    NeedProgressData,
    UpdateProgressData(BudgetProgress),
    NeedCategorySpend,
    UpdateCategorySpend(Vec<CategorySpend>),
    NeedCategories,
    UpdateCategories(ConfigOptions),
}

pub struct BudgetComponent {
    budget_progress: Option<BudgetProgress>,
    category_spend: Option<Arc<Vec<CategorySpend>>>,
    categories: Option<Arc<Vec<String>>>,
}

impl Component for BudgetComponent {
    type Message = BudgetMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Self::Message::NeedCategories);
        ctx.link().send_message(Self::Message::NeedProgressData);

        Self {
            budget_progress: None,
            category_spend: None,
            categories: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BudgetMsg::NeedCategories => ctx.link().send_future(async move {
                BudgetMsg::UpdateCategories(api::get_config("budget_items").await)
            }),
            BudgetMsg::UpdateCategories(config) => match config {
                ConfigOptions::BudgetItems(c) => {
                    self.categories = Some(Arc::new(c));
                    ctx.link().send_message(Self::Message::NeedCategorySpend);
                }
                _ => panic!("wrong config option variant"),
            },
            BudgetMsg::NeedProgressData => {
                let options = BudgetProgressOptions {
                    date: Utc::now().date_naive(),
                };
                ctx.link().send_future(async move {
                    BudgetMsg::UpdateProgressData(api::budget_progress(&options).await)
                });
            }
            BudgetMsg::UpdateProgressData(spend) => {
                self.budget_progress = Some(spend);
            }
            BudgetMsg::NeedCategorySpend => {
                let Some(categories) = &self.categories else {
                    ctx.link().send_message(BudgetMsg::NeedCategories);
                    return false;
                };
                let options = CategorySpendOptions {
                    date: Utc::now().date_naive(),
                    l1_tags: categories.clone().to_vec(),
                };
                ctx.link().send_future(async move {
                    BudgetMsg::UpdateCategorySpend(api::category_spend(&options).await)
                });
            }
            BudgetMsg::UpdateCategorySpend(category) => {
                self.category_spend = Some(Arc::new(category));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Some(budget_progress) = &self.budget_progress else {
            return html! {<></>};
        };

        let Some(category_spend) = &self.category_spend else {
            return html! {<></>};
        };
        let progress = budget_progress.progress() * 100.0;
        let progress = format!("{:.2}%", progress);

        let spent = budget_progress.spend.unwrap_or(0.0);
        let spent = format!("£{:.2}", spent);

        let today = Utc::now().date_naive();
        let expected = progress_through_month(today) * 100.0;
        let expected = format!("{:.2}%", expected);

        html! {
        <>
        <div class="row">
            <div class="wrapper">
                <div class="info"><h2>{"Spent "} {spent}</h2></div>
                <div class="info"><h2>{"Progress "} {progress}</h2></div>
                <div class="info"><h2>{"Expected "} {expected}</h2></div>
            </div>
        </div>
        <div class="row">
        <div class="halves">
            <div id="chart" class="chart">
            <BudgetProgressChartComponent budget_progress={*budget_progress} />
            </div>
        </div>
        <div class="halves">
            <div id="chart2" class="chart2">
            <CategorySpendComponent {category_spend} />
            </div>
        </div>
        </div>
        </>
        }
    }
}

fn progress_through_month(date: NaiveDate) -> f64 {
    let start_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
    let next_month = NaiveDate::from_ymd_opt(
        match date.month() {
            12 => date.year() + 1,
            _ => date.year(),
        },
        match date.month() {
            12 => 1,
            _ => date.month() + 1,
        },
        1,
    )
    .unwrap();
    let number_of_days = next_month.signed_duration_since(start_month).num_days();

    date.day() as f64 / number_of_days as f64
}

#[derive(Properties, PartialEq)]
pub struct BudgetBarPlotProps {
    budget_progress: BudgetProgress,
}

#[function_component(BudgetProgressChartComponent)]
pub fn budget_progress_chart_component(
    BudgetBarPlotProps { budget_progress }: &BudgetBarPlotProps,
) -> Html {
    let mut plot = Plot::new();
    let spend_trace =
        Bar::new(vec!["Budget"], vec![budget_progress.spend.unwrap_or(0.0)]).name("Budget spend");
    plot.add_trace(spend_trace);

    let mut layout = Layout::new().title("Monthly Budget Spend Progress".into());

    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(-0.5)
            .x1(0.5)
            .y0(budget_progress.budget)
            .y1(budget_progress.budget)
            .line(
                ShapeLine::new()
                    .color(NamedColor::Red)
                    .width(4.0)
                    .dash(DashType::Dash),
            ),
    );

    plot.set_layout(layout);
    html! { <Plotly plot={plot}/> }
}

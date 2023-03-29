use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use common::{
    BudgetProgress, BudgetProgressOptions, CategorySpend, CategorySpendOptions, ConfigOptions,
};
use plotly::{Bar, Plot};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_plotly::Plotly;

use crate::api;

pub enum FieldMsg {
    Update(String),
}

#[derive(PartialEq, Properties)]
pub struct DatePickerProps {
    pub id: String,
    pub given_date: String,
    pub on_input: Callback<String>,
}

pub struct DatePicker;

impl Component for DatePicker {
    type Message = FieldMsg;
    type Properties = DatePickerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMsg::Update(d) => ctx.props().on_input.emit(d),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                type="date"
                class="form-control"
                id="date"
                form={ctx.props().id.clone()}
                value={ctx.props().given_date.clone()}
                onchange={ ctx.link().callback(|e: Event| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    log::info!("Date: {}", input.value());
                    FieldMsg::Update(input.value())
                }) }
            />
        }
    }
}

pub enum MonthlyMsg {
    UpdateDate(String),
    NeedProgressData,
    UpdateProgressData(BudgetProgress),
    NeedCategorySpend,
    UpdateCategorySpend(Vec<CategorySpend>),
    NeedCategories,
    UpdateCategories(ConfigOptions),
}

pub struct MonthlyComponent {
    date: NaiveDate,
    budget_progress: Option<BudgetProgress>,
    category_spend: Option<Arc<Vec<CategorySpend>>>,
    categories: Option<Arc<Vec<String>>>,
}

impl Component for MonthlyComponent {
    type Message = MonthlyMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Self::Message::NeedCategories);
        ctx.link().send_message(Self::Message::NeedProgressData);

        Self {
            date: Utc::now().date_naive(),
            budget_progress: None,
            category_spend: None,
            categories: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MonthlyMsg::UpdateDate(d) => {
                self.date = NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap();
                ctx.link().send_message(Self::Message::NeedProgressData);
                ctx.link().send_message(Self::Message::NeedCategorySpend);
            }
            MonthlyMsg::NeedCategories => ctx.link().send_future(async move {
                MonthlyMsg::UpdateCategories(api::get_config("period_items").await)
            }),
            MonthlyMsg::UpdateCategories(config) => match config {
                ConfigOptions::PeriodItems(c) => {
                    self.categories = Some(Arc::new(c));
                    ctx.link().send_message(Self::Message::NeedCategorySpend);
                }
                _ => panic!("wrong config option variant"),
            },
            MonthlyMsg::NeedProgressData => {
                let options = BudgetProgressOptions { date: self.date };
                ctx.link().send_future(async move {
                    MonthlyMsg::UpdateProgressData(api::budget_progress(&options).await)
                });
            }
            MonthlyMsg::UpdateProgressData(spend) => {
                self.budget_progress = Some(spend);
            }
            MonthlyMsg::NeedCategorySpend => {
                let Some(categories) = &self.categories else {
                    ctx.link().send_message(MonthlyMsg::NeedCategories);
                    return false;
                };
                let options = CategorySpendOptions {
                    date: self.date,
                    l1_tags: categories.clone().to_vec(),
                };
                ctx.link().send_future(async move {
                    MonthlyMsg::UpdateCategorySpend(api::category_spend(&options).await)
                });
            }
            MonthlyMsg::UpdateCategorySpend(category) => {
                self.category_spend = Some(Arc::new(category));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = "month".to_string();
        let given_date = self.date.to_string();
        let date_form = html! {
            <>
            <DatePicker id={id} {given_date}
            on_input={ctx.link().callback(MonthlyMsg::UpdateDate)}/>
            </>
        };
        let Some(budget_progress) = &self.budget_progress else {
            return date_form;
        };

        let Some(category_spend) = &self.category_spend else {
            return date_form;
        };
        let spent = budget_progress.spend.unwrap_or(0.0);
        let spent = format!("£{:.2}", spent);

        let total_spend: f64 = category_spend.iter().filter_map(|c| c.amount).sum();
        let total_spend = format!("£{:.2}", total_spend);

        html! {
        <>
        {date_form}
        <div class="row">
            <div class="wrapper">
                <div class="info"><h2>{"Budget Spend "} {spent}</h2></div>
                <div class="info"><h2>{"Total Spend "} {total_spend}</h2></div>
            </div>
        </div>
        <div class="row">
        <div id="chart" class="chart">
            <CategorySpendComponent {category_spend} />
        </div>
        </div>
        </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CategorySpendProps {
    category_spend: Arc<Vec<CategorySpend>>,
}

#[function_component(CategorySpendComponent)]
pub fn category_spend_component(
    CategorySpendProps { category_spend }: &CategorySpendProps,
) -> Html {
    let mut categories = vec![];
    let mut spend = vec![];
    for c in category_spend.iter() {
        categories.push(c.name.clone());
        spend.push(c.amount);
    }
    log::info!("{:?}", spend);
    let mut plot = Plot::new();
    let spend_trace = Bar::new(categories, spend);
    plot.add_trace(spend_trace);

    html! { <Plotly plot={plot}/> }
}
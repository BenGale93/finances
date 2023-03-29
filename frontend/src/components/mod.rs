use std::sync::Arc;

use common::CategorySpend;
use plotly::{Bar, Layout, Plot};
use yew::prelude::*;
use yew_plotly::Plotly;

#[derive(Properties, PartialEq)]
pub struct CategorySpendProps {
    pub category_spend: Arc<Vec<CategorySpend>>,
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

    let mut plot = Plot::new();
    let spend_trace = Bar::new(categories, spend).name("Category Spend");

    plot.add_trace(spend_trace);

    let layout = Layout::new().title("Monthly Category Spend".into());

    plot.set_layout(layout);

    html! { <Plotly plot={plot}/> }
}

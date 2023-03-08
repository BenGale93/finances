use std::sync::Arc;

use common::{BalanceByDay, BalanceOverTime, BalancesByDayExt};
use plotly::{Plot, Scatter};
use yew::prelude::*;

use crate::api;

pub enum BalanceMsg {
    NeedUpdateBalanceOverTime,
    UpdateBalanceOverTime(Vec<BalanceByDay>),
}

pub struct BalanceComponent {
    balance_over_time: Option<Arc<Vec<BalanceByDay>>>,
}

impl Component for BalanceComponent {
    type Message = BalanceMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            balance_over_time: None,
        };

        ctx.link()
            .send_message(Self::Message::NeedUpdateBalanceOverTime);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BalanceMsg::NeedUpdateBalanceOverTime => {
                log::info!("Updating balance over time.");
                ctx.link().send_future(async move {
                    BalanceMsg::UpdateBalanceOverTime(api::balance_over_time().await)
                });
            }
            BalanceMsg::UpdateBalanceOverTime(balance_over_time) => {
                self.balance_over_time = Some(Arc::new(balance_over_time));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Some(balance_over_time) = &self.balance_over_time else {
            return html! {<></>};
        };
        let db = balance_over_time.cumsum();
        let ma = balance_over_time.rolling_average_cumsum(30);
        log::info!("Data computed.");

        html! {<><h1>{"Balance"}</h1><BalanceOverTimeComponent daily_balance={db} ma={ma}/></>}
    }
}

#[derive(Properties, PartialEq)]
pub struct BalancePlotProps {
    pub daily_balance: BalanceOverTime,
    pub ma: Option<BalanceOverTime>,
}

#[function_component(BalanceOverTimeComponent)]
pub fn balance_component(BalancePlotProps { daily_balance, ma }: &BalancePlotProps) -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();
        let trace = Scatter::new(
            daily_balance.dates.to_owned(),
            daily_balance.balances.to_owned(),
        );
        plot.add_trace(trace);

        match ma {
            Some(ma_balance) => {
                let ma_trace =
                    Scatter::new(ma_balance.dates.to_owned(), ma_balance.balances.to_owned());
                plot.add_trace(ma_trace);
            }
            None => (),
        };

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    use_effect_with_deps(
        move |_| {
            p.run();
            || ()
        },
        (),
    );

    html! {<div id="plot-div"></div>}
}

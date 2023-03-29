use std::sync::Arc;

use common::{BalanceByTime, BalanceOverTime, BalancesByDayExt, DateGrouping};
use plotly::{layout::BarMode, Bar, Layout, Plot, Scatter};
use yew::prelude::*;

use crate::api;

pub enum BalanceMsg {
    NeedUpdateBalance,
    UpdateBalanceByDay(Vec<BalanceByTime>),
    UpdateBalanceByMonth(Vec<BalanceByTime>),
}

pub struct BalanceComponent {
    balance_by_day: Option<Arc<Vec<BalanceByTime>>>,
    balance_by_month: Option<Arc<Vec<BalanceByTime>>>,
}

impl Component for BalanceComponent {
    type Message = BalanceMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            balance_by_day: None,
            balance_by_month: None,
        };

        ctx.link().send_message(Self::Message::NeedUpdateBalance);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BalanceMsg::NeedUpdateBalance => {
                log::info!("Updating balance by day.");
                ctx.link().send_future(async move {
                    BalanceMsg::UpdateBalanceByDay(api::balance_by_date(DateGrouping::Day).await)
                });
                ctx.link().send_future(async move {
                    BalanceMsg::UpdateBalanceByMonth(
                        api::balance_by_date(DateGrouping::Month).await,
                    )
                });
            }
            BalanceMsg::UpdateBalanceByDay(balance_over_time) => {
                self.balance_by_day = Some(Arc::new(balance_over_time));
            }
            BalanceMsg::UpdateBalanceByMonth(balance_over_time) => {
                self.balance_by_month = Some(Arc::new(balance_over_time));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Some(balance_by_day) = &self.balance_by_day else {
            return html! {<></>};
        };
        let Some(balance_by_month) = &self.balance_by_month else {
            return html! {<></>};
        };

        let db = balance_by_day.cumsum();
        let ma = balance_by_day.rolling_average_cumsum(30);

        let (months, monthly_incoming, monthly_outgoing, monthly_balance) =
            balance_by_month.vectors();

        html! {
            <>
            <BalanceOverTimeComponent daily_balance={db} ma={ma}/>
            <BalanceByMonthComponent {months} {monthly_incoming} {monthly_outgoing} {monthly_balance}/>
            </>
        }
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

#[derive(Properties, PartialEq)]
pub struct BalanceBarPlotProps {
    pub months: Vec<String>,
    pub monthly_incoming: Vec<f64>,
    pub monthly_outgoing: Vec<f64>,
    pub monthly_balance: Vec<f64>,
}

#[function_component(BalanceByMonthComponent)]
pub fn balance_by_month_component(
    BalanceBarPlotProps {
        months,
        monthly_outgoing,
        monthly_incoming,
        monthly_balance,
    }: &BalanceBarPlotProps,
) -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "bar-div";
        let mut plot = Plot::new();
        let out_trace = Bar::new(months.to_owned(), monthly_outgoing.to_owned());
        plot.add_trace(out_trace);
        let in_trace = Bar::new(months.to_owned(), monthly_incoming.to_owned());
        plot.add_trace(in_trace);
        let total_trace = Scatter::new(months.to_owned(), monthly_balance.to_owned());
        plot.add_trace(total_trace);

        let layout = Layout::new().bar_mode(BarMode::Overlay);

        plot.set_layout(layout);

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

    html! {<div id="bar-div"></div>}
}

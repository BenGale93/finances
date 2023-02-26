use std::sync::Arc;

use common::AccountSummary;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct AccountComponentProps {
    pub account: AccountSummary,
}

#[function_component(AccountComponent)]
fn account_component(props: &AccountComponentProps) -> Html {
    let AccountComponentProps { account } = props;
    let amount = account.amount;
    let total = format!("{amount:.2}");
    html! {
        <tr>
            <td class="account">{account.name.to_owned()}</td>
            <td class="date">{total}</td>
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct AccountsComponentProps {
    pub accounts: Arc<Vec<AccountSummary>>,
}

pub struct AccountSummaryComponent {}

impl Component for AccountSummaryComponent {
    type Message = ();
    type Properties = AccountsComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props()
            .accounts
            .iter()
            .map(|account| {
                html! {
                <AccountComponent account={account.clone()}/>
                }
            })
            .collect()
    }
}

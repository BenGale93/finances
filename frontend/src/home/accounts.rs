use std::sync::Arc;

use common::AccountSummary;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct AccountComponentProps {
    pub account: AccountSummary,
}

#[function_component(AccountComponent)]
fn account_component(AccountComponentProps { account }: &AccountComponentProps) -> Html {
    let amount = account.amount;
    let total = format!("{amount:.2}");
    html! {
        <tr>
            <td>{account.name.to_owned()}</td>
            <td>{total}</td>
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct AccountsComponentProps {
    pub accounts: Arc<Vec<AccountSummary>>,
}

#[function_component(AccountsSummaryComponent)]
pub fn accounts_summary_component(
    AccountsComponentProps { accounts }: &AccountsComponentProps,
) -> Html {
    accounts
        .iter()
        .map(|account| {
            html! {
            <AccountComponent account={account.clone()}/>
            }
        })
        .collect()
}

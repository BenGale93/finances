use common::AccountSummary;
use reqwasm::http::Request;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct AccountComponentProps {
    pub account: AccountSummary,
}

#[function_component(AccountComponent)]
fn account_component(props: &AccountComponentProps) -> Html {
    let AccountComponentProps { account } = props;
    html! {
        <tr>
            <td class="account">{account.name.to_owned()}</td>
            <td class="date">{account.amount.to_owned()}</td>
        </tr>
    }
}

pub struct AccountSummaryComponent {
    accounts: Option<Vec<AccountSummary>>,
}

fn get_accounts(accounts_cb: Callback<Vec<AccountSummary>>) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let fetched_accounts = Request::get("http://localhost:5000/accounts")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            accounts_cb.emit(fetched_accounts);
        }
    })
}

pub enum AccountsMsg {
    Load(Vec<AccountSummary>),
}

impl Component for AccountSummaryComponent {
    type Message = AccountsMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let account_cb = ctx.link().callback(AccountsMsg::Load);
        get_accounts(account_cb);
        Self { accounts: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountsMsg::Load(a) => {
                self.accounts = Some(a);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.accounts {
            Some(f) => f
                .iter()
                .map(|account| {
                    html! {
                    <AccountComponent account={account.clone()}/>
                    }
                })
                .collect(),
            None => {
                html! {
                    <>
                        {"Loading accounts"}
                    </>
                }
            }
        }
    }
}

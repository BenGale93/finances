mod accounts;
mod transactions;

use yew::{function_component, html, Html};

use crate::home::{accounts::AccountSummaryComponent, transactions::TransactionsComponent};

#[function_component(HomeComponent)]
pub fn home_component() -> Html {
    html! {
        <div>
        <div class="row">
        <div class="column left">
            <h2>{"Accounts"}</h2>
        </div>
        <div class="column right">
            <div class="wrapper">
                <div class="info"><h2>{"Total"}</h2></div>
                <div class="info"><h2>{"Budget"}</h2></div>
            </div>
        </div>
    </div>
    <div class="row">
        <div class="input_tran">
        {"add transaction TODO"}
        </div>
    </div>
    <div class="row">
        <div class="column left">
        <table>
        <tr>
            <th>{"Account"}</th>
            <th>{"Amount"}</th>
        </tr>
        <AccountSummaryComponent />
        </table>
        </div>
        <div class="column right">
        <table>
        <tr>
            <th>{"Account"}</th>
            <th>{"Date"}</th>
            <th>{"Description"}</th>
            <th>{"Amount"}</th>
            <th>{"L1 Tag"}</th>
            <th>{"L2 Tag"}</th>
            <th>{"L3 Tag"}</th>
        </tr>
        <TransactionsComponent />
        </table>
        </div>
    </div>
    </div>
    }
}

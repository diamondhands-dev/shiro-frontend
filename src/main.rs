use yew::prelude::*;
use yew_router::prelude::*;

use self::components::balance_tab::BalanceTab;
use self::components::bitcoin_page::Page as BitcoinPage;
use self::components::issue_asset_page::Page as IssueAssetPage;
use self::components::mnemonic_page::Page as MnemonicPage;
use self::components::utxos_page::Page as UtxosPage;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    BalanceTabRoute,
    #[at("/bitcoin")]
    BitcoinPageRoute,
    #[at("/issue")]
    IssueAssetPageRoute,
    #[at("/mnemonic")]
    MnemonicPageRoute,
    #[at("/utxos")]
    UtxosPageRoute,
}

#[function_component(BalanceTabRoute)]
fn balance_tab() -> Html {
    html! {
        <section>
            <BalanceTab/>
        </section>
    }
}

#[function_component(BitcoinPageRoute)]
fn bitcoin_page() -> Html {
    html! {
        <section>
            <BitcoinPage/>
        </section>
    }
}

#[function_component(MnemonicPageRoute)]
fn utxo_page() -> Html {
    html! {
        <section>
            <MnemonicPage/>
        </section>
    }
}

#[function_component(IssueAssetPageRoute)]
fn issue_asset_page() -> Html {
    html! {
        <section>
            <IssueAssetPage/>
        </section>
    }
}

#[function_component(UtxosPageRoute)]
fn mnemonic_page() -> Html {
    html! {
        <section>
            <UtxosPage/>
        </section>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::BalanceTabRoute => html!( <BalanceTabRoute/> ),
        Route::BitcoinPageRoute => html!( <BitcoinPageRoute/> ),
        Route::IssueAssetPageRoute => html!( <IssueAssetPageRoute/> ),
        Route::MnemonicPageRoute => html!( <MnemonicPageRoute/> ),
        Route::UtxosPageRoute => html!( <UtxosPageRoute/> ),
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <div>
            <div class="navbar navbar-default">
                <a class="navbar-brand" href="/">{"Shiro-wallet"}</a>
                <a class="navbar-link" href="bitcoin">{"bitcoin"}</a>
                <a class="navbar-link" href="issue">{"issue"}</a>
                <a class="navbar-link" href="mnemonic">{"mnemonic"}</a>
                <a cless="navbar-link" href="utxos">{"utxos"}</a>
            </div>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
}

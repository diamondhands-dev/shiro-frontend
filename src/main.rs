use material_yew::MatIcon;
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
        Route::BalanceTabRoute => html!( <>{inject_navbar()} <BalanceTabRoute/></> ),
        Route::BitcoinPageRoute => html!( <>{inject_navbar()} <BitcoinPageRoute/></> ),
        Route::IssueAssetPageRoute => html!( <>{inject_navbar()} <IssueAssetPageRoute/></> ),
        Route::MnemonicPageRoute => html!( <>{inject_navbar()} <MnemonicPageRoute/></> ),
        Route::UtxosPageRoute => html!( <>{inject_navbar()} <UtxosPageRoute/></> ),
    }
}

fn inject_navbar() -> Html {
    html! {
                <nav class="navbar navbar-expand-lg">
                    <div class="container">
                        <a class="navbar-brand" href="javascript:void(0)">{"Shiro-wallet"}</a>
                        <button class="navbar-toggler ms-auto" type="button" data-bs-toggle="collapse" data-bs-target="#n_bar" aria-controls="navbarNavAltMarkup" aria-label="Toggle navigation">
                            <MatIcon>{"menu"}</MatIcon>
                        </button>
                        <div class="collapse navbar-collapse" id="n_bar">
                            <ul class="navbar-nav active">
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::MnemonicPageRoute}>{"Mnemonic"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::BalanceTabRoute}>{"Balnce"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::BitcoinPageRoute}>{"Bitcoin"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::IssueAssetPageRoute}>{"Issue"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::UtxosPageRoute}>{"UTXOs"}</Link<Route>></li>
                            </ul>
                        </div>
                    </div>
                </nav>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
            <>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM" crossorigin="anonymous"></script>
            </>
        }
}

fn main() {
    yew::start_app::<Main>();
    wasm_logger::init(wasm_logger::Config::default());
}

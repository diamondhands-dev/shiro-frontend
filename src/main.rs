#![allow(clippy::let_unit_value)]

use material_yew::MatIcon;
use yew::prelude::*;
use yew_router::prelude::*;

use self::components::asset_balance_page::Page as AssetBalancePage;
use self::components::asset_receive_page::Page as AssetReceivePage;
use self::components::asset_send_page::Page as AssetSendPage;
use self::components::asset_list_page::Page as AssetListPage;
use self::components::bitcoin_page::Page as BitcoinPage;
use self::components::issue_asset_page::Page as IssueAssetPage;
use self::components::mnemonic_page::Page as MnemonicPage;
use self::components::utxos_page::Page as UtxosPage;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Mnemonic,
    #[at("/bitcoin")]
    Bitcoin,
    #[at("/issue")]
    IssueAsset,
    #[at("/utxos")]
    Utxos,
    #[at("/balance")]
    Balance,
    #[at("/balance/receive/:asset_id")]
    AssetReceive { asset_id: String },
    #[at("/balance/send/:asset_id")]
    AssetSend { asset_id: String },
    #[at("/balance/:asset_id")]
    AssetBalance { asset_id: String },
}

#[derive(Properties, PartialEq)]
pub struct AssetBalancePageRouteProp {
    asset_id: String,
}

#[function_component(AssetBalancePageRoute)]
fn asset_balance_page(prop: &AssetBalancePageRouteProp) -> Html {
    html! {
        <>
            {inject_navbar()}
            <AssetBalancePage asset_id={prop.asset_id.clone()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct AssetReceivePageRouteProp {
    asset_id: String,
}

#[function_component(AssetReceivePageRoute)]
fn asset_receive_page(prop: &AssetReceivePageRouteProp) -> Html {
    html! {
        <>
            {inject_navbar()}
            <AssetReceivePage asset_id={prop.asset_id.clone()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct AssetSendPageRouteProp {
    asset_id: String,
}

#[function_component(AssetSendPageRoute)]
fn asset_send_page(prop: &AssetSendPageRouteProp) -> Html {
    html! {
        <>
            {inject_navbar()}
            <AssetSendPage asset_id={prop.asset_id.clone()}/>
        </>
    }
}

#[function_component(AssetListPageRoute)]
fn asset_list() -> Html {
    html! {
        <>
            {inject_navbar()}
            <AssetListPage/>
        </>
    }
}

#[function_component(BitcoinPageRoute)]
fn bitcoin_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <BitcoinPage/>
        </>
    }
}

#[function_component(MnemonicPageRoute)]
fn mnemonic_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <MnemonicPage/>
        </>
    }
}

#[function_component(IssueAssetPageRoute)]
fn issue_asset_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <IssueAssetPage/>
        </>
    }
}

#[function_component(UtxosPageRoute)]
fn utxos_page() -> Html {
    html! {
        <>
            {inject_navbar()}
            <UtxosPage/>
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Balance => html!( <AssetListPageRoute/> ),
        Route::Bitcoin => html!( <BitcoinPageRoute/> ),
        Route::IssueAsset => html!( <IssueAssetPageRoute/> ),
        Route::Mnemonic => html!( <MnemonicPageRoute/> ),
        Route::Utxos => html!( <UtxosPageRoute/> ),
        Route::AssetBalance { asset_id } => {
            html!( <AssetBalancePageRoute asset_id={asset_id.clone()} /> )
        }
        Route::AssetReceive { asset_id } => {
            html!( <AssetReceivePageRoute asset_id={asset_id.clone()} /> )
        }
        Route::AssetSend { asset_id } => {
            html!( <AssetSendPageRoute asset_id={asset_id.clone()} /> )
        }
    }
}

fn inject_navbar() -> Html {
    html! {
                <nav class="navbar navbar-expand-lg">
                    <div class="container">
                        <a class="navbar-brand" href="./balance">{"Shiro-wallet"}</a>
                        <button class="navbar-toggler ms-auto" type="button" data-bs-toggle="collapse" data-bs-target="#n_bar" aria-controls="navbarNavAltMarkup" aria-label="Toggle navigation">
                            <MatIcon>{"menu"}</MatIcon>
                        </button>
                        <div class="collapse navbar-collapse" id="n_bar">
                            <ul class="navbar-nav active">
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::Mnemonic}>{"Mnemonic"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::Balance}>{"Asset"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::Bitcoin}>{"Bitcoin"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::IssueAsset}>{"Issue"}</Link<Route>></li>
                                 <li><Link<Route> classes={classes!("nav-link")} to={Route::Utxos}>{"UTXOs"}</Link<Route>></li>
                            </ul>
                        </div>
                    </div>
                </nav>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
    wasm_logger::init(wasm_logger::Config::default());
}

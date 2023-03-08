use super::common::RefreshButton;
use crate::components::asset_balance_page::ReceiveButton;
use crate::Route;
use ::material_yew::{MatTab, MatTabBar};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};
use yew_router::prelude::Link;

/// The type of an asset
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AssetType {
    /// Rgb20 schema for fungible assets
    Rgb20,
    /// Rgb121 schema for non-fungible assets
    Rgb121,
}

#[derive(Deserialize, Serialize)]
pub struct AssetsParams {
    pub filter_asset_types: Vec<AssetType>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Balance {
    pub settled: String,
    pub future: String,
    pub spendable: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Media {
    file_path: String,
    mime: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetRgb20 {
    pub asset_id: String,
    pub ticker: String,
    pub name: String,
    pub precision: u8,
    pub balance: Balance,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetRgb121 {
    pub asset_id: String,
    pub name: String,
    pub precision: u8,
    pub description: Option<String>,
    pub balance: Balance,
    pub data_paths: Vec<Media>,
    pub parent_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Assets {
    pub rgb20: Vec<AssetRgb20>,
    pub rgb121: Vec<AssetRgb121>,
}

#[derive(Deserialize, Serialize)]
pub struct AssetsResult {
    pub assets: Assets,
}

enum Tabs {
    Fungible,
    NFT,
}

#[derive(Properties, PartialEq)]
pub struct BalancePageProps {}

#[function_component(BalancePageInner)]
pub fn page(_props: &BalancePageProps) -> Html {
    let tab = use_state(|| Tabs::Fungible);
    let fungible_list = use_state(|| Vec::<AssetRgb20>::new());
    let nft_list = use_state(|| Vec::<AssetRgb121>::new());

    let on_activated = {
        let tab = tab.clone();
        Callback::from(move |index| match index {
            0 => tab.set(Tabs::Fungible),
            1 => tab.set(Tabs::NFT),
            num => unreachable!("{}", num),
        })
    };

    let content = {
        let client = reqwest::Client::new();
        let f_list = fungible_list.clone();
        let n_list = nft_list.clone();
        spawn_local(async move {
            let res = client
                .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/assets")
                .json(&AssetsParams {
                    filter_asset_types: Vec::<AssetType>::new(),
                })
                .send()
                .await;
            match res {
                Ok(res) => match res.json::<AssetsResult>().await {
                    Ok(json) => {
                        f_list.set(json.assets.rgb20);
                        //mock start
                        f_list.set(vec![AssetRgb20 {
                            asset_id: "hoge".to_string(),
                            ticker: "FAKEMONA".to_string(),
                            name: "Fake Monacoin".to_string(),
                            precision: 8,
                            balance: Balance {
                                settled: "0".to_string(),
                                future: "0".to_string(),
                                spendable: "1".to_string(),
                            },
                        }]);
                        //mock end
                        n_list.set(json.assets.rgb121);
                        log::info!("Got assets");
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                },
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        });
        match *tab {
            Tabs::Fungible => html! {
                <>
                {
                    (*fungible_list).iter().enumerate().map(|(_,asset)| {
                        let spendable = asset.balance.spendable.clone().parse::<f64>().unwrap();
                        html! {
                            <Link<Route> to={Route::AssetBalancePageRoute {asset_id: asset.asset_id.clone()}}>
                            <div class="container">
                            <div> {asset.asset_id.clone()} </div>
                            <div> {asset.ticker.clone()} {"("} {asset.name.clone()} {")"}</div>
                            <div> {spendable / 10f64.powi(asset.precision as i32)} </div>
                            </div>
                            </Link<Route>>
                        }}).collect::<Html>()
                }
                </>
            },
            Tabs::NFT => html! {
                <>
                {
                    (*nft_list).iter().enumerate().map(|(_,asset)| {
                        let spendable = asset.balance.spendable.clone().parse::<f64>().unwrap();
                        html! {
                            <div class="container">
                                <div>{asset.asset_id.clone()}</div>
                                <div>{asset.name.clone()}</div>
                                <div>{spendable / 10f64.powi(asset.precision as i32)}</div>
                                <div> {match asset.description.clone() {
                                    Some(description) => description,
                                    None => "".to_string(),
                                }}</div>
                            </div>
                        }}).collect::<Html>()
                }
                </>
            },
        }
    };

    html! {
        <>
            <MatTabBar onactivated={on_activated}>
                <MatTab icon="wallet" label="Fungible" />
                <MatTab icon="image" label="NFT" />
            </MatTabBar>
            {content}
            <ReceiveButton label={"RECEIVE ASSETS"} asset_id={"_"}/>
            <RefreshButton/>
        </>
    }
}

pub struct Page {}

impl Component for Page {
    type Properties = ();
    type Message = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <BalancePageInner/>
        }
    }
}

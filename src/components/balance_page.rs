use super::common::RefreshButton;
use crate::components::asset_balance_page::ReceiveButton;
use crate::Route;
use ::material_yew::{MatTab, MatTabBar};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties, Component, Context};
use yew_router::prelude::Link;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

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

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Balance {
    pub settled: String,
    pub future: String,
    pub spendable: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Media {
    file_path: String,
    mime: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct AssetRgb20 {
    pub asset_id: String,
    pub ticker: String,
    pub name: String,
    pub precision: u8,
    pub balance: Balance,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct AssetRgb121 {
    pub asset_id: String,
    pub name: String,
    pub precision: u8,
    pub description: Option<String>,
    pub balance: Balance,
    pub data_paths: Vec<Media>,
    pub parent_id: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
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
    let fungible_list = use_state_eq(|| Vec::<AssetRgb20>::new());
    let nft_list = use_state_eq(|| Vec::<AssetRgb121>::new());

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
                //.put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/assets")
                .put(API_ROOT.unwrap_or("http://localhost:8080").to_owned() + "/wallet/assets")
                .json(&AssetsParams {
                    filter_asset_types: Vec::<AssetType>::new(),
                })
                .send()
                .await;
            match res {
                Ok(res) => match res.json::<AssetsResult>().await {
                    Ok(json) => {
                        f_list.set(json.assets.rgb20);
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
                <div class="list-group">
                {
                    (*fungible_list).iter().enumerate().map(|(_,asset)| {
                        let spendable = asset.balance.spendable.clone().parse::<f64>().unwrap();
                        html! {
                            <a href="#" class="list-group-item list-group-item-action flex-column align-items-start">
                            <Link<Route> to={Route::AssetBalancePageRoute {asset_id: asset.asset_id.clone()}}>
                            <div class="d-flex w-100 justify-content-between">
                            <h5 class="mb-1">{asset.name.clone()}</h5>
                            <small>{spendable / 10f64.powi(asset.precision as i32)}</small>
                            </div>
                            <p class="mb-1 truncate">{asset.asset_id.clone()}</p>
                            </Link<Route>>
                            </a>
                        }}).collect::<Html>()
                }
                </div>
                </>
            },
            Tabs::NFT => html! {
                <>
                {
                    (*nft_list).iter().enumerate().map(|(_,asset)| {
                        let spendable = asset.balance.spendable.clone().parse::<f64>().unwrap();
                        html! {
                            // FIXME
                            <div class="container">
                                //<div>{asset.asset_id.clone()}</div>
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

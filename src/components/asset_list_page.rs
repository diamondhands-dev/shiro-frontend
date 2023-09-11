use super::common::RefreshButton;
use crate::components::asset_balance_page::ReceiveButton;
use crate::Route;
use ::material_yew::{MatTab, MatTabBar};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Component, Context, Html, Properties};
use yew_router::prelude::Link;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

/// The type of an asset
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AssetType {
    /// Rgb20 schema for fungible assets
    Rgb20,
    /// Rgb25 schema for non-fungible assets
    Rgb25,
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
pub struct AssetRgb25 {
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
    pub rgb25: Vec<AssetRgb25>,
}

#[derive(Deserialize, Serialize)]
pub struct AssetsResult {
    pub assets: Assets,
}

enum Tabs {
    Fungible,
    Nft,
}

#[derive(Properties, PartialEq)]
pub struct BalancePageProps {}

#[function_component(BalancePageInner)]
pub fn page(_props: &BalancePageProps) -> Html {
    let baseurl = web_sys::window().unwrap().origin();
    let message = use_state(|| "".to_string());
    let tab = use_state(|| Tabs::Fungible);
    let fungible_list = use_state_eq(Vec::<AssetRgb20>::new);
    let nft_list = use_state_eq(Vec::<AssetRgb25>::new);

    let on_activated = {
        let tab = tab.clone();
        Callback::from(move |index| match index {
            0 => tab.set(Tabs::Fungible),
            1 => tab.set(Tabs::Nft),
            num => unreachable!("{}", num),
        })
    };

    let content = {
        let message = message.clone();
        let client = reqwest::Client::new();
        let f_list = fungible_list.clone();
        let n_list = nft_list.clone();
        spawn_local(async move {
            let res = client
                .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/assets")
                .json(&AssetsParams {
                    filter_asset_types: Vec::<AssetType>::new(),
                })
                .send()
                .await;
            match res {
                Ok(res) => {
                    let res_text = res.text().await.unwrap();
                    match serde_json::from_str::<AssetsResult>(&res_text) {
                        //match res.json::<AssetsResult>().await {
                        Ok(json) => {
                            f_list.set(json.assets.rgb20);
                            n_list.set(json.assets.rgb25);
                            log::info!("Got assets");
                        }
                        Err(e) => {
                            log::error!("{:?}", e);
                            message.set(res_text)
                        }
                    }
                }
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
                        let future = asset.balance.future.clone().parse::<f64>().unwrap();
                        html! {
                            <a href="#" class="list-group-item list-group-item-action flex-column align-items-start">
                            <Link<Route> to={Route::AssetBalance {asset_id: asset.asset_id.clone()}}>
                            <div class="d-flex w-100 justify-content-between">
                            <h5 class="mb-1">{asset.name.clone()}</h5>
                            <small>{future / 10f64.powi(asset.precision as i32)}</small>
                            </div>
                            <p class="mb-1 truncate">{asset.asset_id.clone()}</p>
                            </Link<Route>>
                            </a>
                        }}).collect::<Html>()
                }
                </div>
                </>
            },
            Tabs::Nft => html! {
                <>
                {
                    (*nft_list).iter().enumerate().map(|(_,asset)| {
                        let future = asset.balance.future.clone().parse::<f64>().unwrap();
                        html! {
                            // FIXME
                            <div class="container">
                                //<div>{asset.asset_id.clone()}</div>
                                <div>{asset.name.clone()}</div>
                                <div>{future / 10f64.powi(asset.precision as i32)}</div>
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

            <div class="mt-4 d-grid gap-2 col-5 mx-auto">
                <ReceiveButton label={"RECEIVE ASSETS"} asset_id={"_"}/>
            </div>

            <RefreshButton/>

            <p class="message">{(*message).to_string()}</p>
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

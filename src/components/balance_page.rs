use ::material_yew::{MatTab, MatTabBar};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

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
    filter_asset_types: Vec<AssetType>,
}

#[derive(Deserialize, Serialize)]
pub struct Balance {
    settled: String,
    future: String,
    spendable: String,
}

#[derive(Deserialize, Serialize)]
pub struct Media {
    file_path: String,
    mime: String,
}

#[derive(Deserialize, Serialize)]
pub struct AssetRgb20 {
    asset_id: String,
    ticker: String,
    name: String,
    precision: u8,
    balance: Balance,
}

#[derive(Deserialize, Serialize)]
pub struct AssetRgb121 {
    asset_id: String,
    name: String,
    precision: u8,
    description: Option<String>,
    balance: Balance,
    data_paths: Vec<Media>,
    parent_id: Option<String>,
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
        let list = fungible_list.clone();
        spawn_local(async move {
            let res = client
                .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/assets")
                .json(&AssetsParams {
                    filter_asset_types: Vec::<AssetType>::new(),
                })
                .send()
                .await;
            list.set(vec![AssetRgb20 {
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
            log::info!("get {:?}", res);
        });
        match *tab {
            Tabs::Fungible => html! {
                <>
                {
                    (*fungible_list).iter().enumerate().map(|(_,asset)| {
                        let spendable = asset.balance.spendable.clone().parse::<f64>().unwrap();
                        html! {
                            <div class="container">
                            <div> {asset.asset_id.clone()} </div>
                            <div> {asset.ticker.clone()} {"("} {asset.name.clone()} {")"}</div>
                            <div> {spendable / 10f64.powi(asset.precision as i32)} </div>
                            </div>
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

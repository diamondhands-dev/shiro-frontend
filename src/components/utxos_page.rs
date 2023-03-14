use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, prelude::*, Html, Properties};
use material_yew::{MatButton, MatTextField, MatCircularProgress};
use yew::virtual_dom::AttrValue;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Outpoint {
    pub txid: String,
    pub vout: u8,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Utxo {
    pub outpoint: Outpoint,
    pub colorable: bool,
    pub btc_amount: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct RgbAllocations {
    pub asset_id: String,
    pub amount: String,
    pub settled: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct UtxoRGB {
    utxo: Utxo,
    rgb_allocations: Vec<RgbAllocations>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct UnspentsResult {
    unspents: Vec<UtxoRGB>,
}

#[derive(Serialize, Deserialize)]
pub struct UnspentsParams {
    settled_only: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UtxosParams {
    up_to: bool,
    num: Option<u16>,
    fee_rate: f32,
}

#[derive(Properties, PartialEq)]
pub struct UtxosListProps {}

#[function_component(UtxosList)]
pub fn utxo_list(_props: &UtxosListProps) -> Html {
    let new_utxo = use_state(|| false);
    let message = use_state(|| "".to_string());

    let utxo_list = use_state_eq(|| Vec::<UtxoRGB>::new());

    let onclick = {
        let message = message.clone();
        let new_utxo = new_utxo.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();
            let utxos = UtxosParams {
                up_to: true,
                num: None,
                fee_rate: 2.0,
            };
            let client = reqwest::Client::new();
            let new_utxo = new_utxo.clone();
            spawn_local(async move {
                new_utxo.set(true);
                let res = client
                    .put(API_ROOT.unwrap_or("http://localhost:8080").to_owned() + "/wallet/utxos")
                    .json(&utxos)
                    .send()
                    .await;
                new_utxo.set(false);
                match res {
                    Ok(res) => {
                        log::info!("{:#?}", res);
                        match res.text().await {
                            Ok(text) => {
                                message.set(text);
                            },
                            Err(e) => {
                                log::error!("{:?}", e);
                                message.set(e.to_string());
                            },
                        }
                    },
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                }
            });
        })
    };

    let onload = {
        let unspents = UnspentsParams {
            settled_only: true,
        };
        let client = reqwest::Client::new();
        let u_list = utxo_list.clone();
        spawn_local(async move {
            let res = client
                .put(API_ROOT.unwrap_or("http://localhost:8080").to_owned() + "/wallet/unspents")
                .json(&unspents)
                .send()
                .await;
            match res {
                Ok(res) => match res.json::<UnspentsResult>().await {
                    Ok(json) => {
                        u_list.set(json.unspents);
                    },
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                },
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        });
        //let utxo_list = json.unspents;
        (*utxo_list).iter().enumerate().map(|(_,utxo_rgb)| {
            //let spendable = utxo.balance.spendable.clone().parse::<f64>().unwrap();
            html! {
                // Passing data `asset_id`
                //<Link<Route> to={Route::AssetBalancePageRoute {asset_id: asset.asset_id.clone()}}>
                <div class="container">
                <div> {utxo_rgb.utxo.outpoint.txid.clone()} </div>
                //<div> {asset.ticker.clone()} {"("} {asset.name.clone()} {")"}</div>
                //<div> {spendable / 10f64.powi(asset.precision as i32)} </div>
                </div>
                //</Link<Route>>
            }}).collect::<Html>()
        //html! {}
    };

    html! {
        <>
            <div>
                {"UTXO List"}
                {onload}
            </div>
            <div>
                if *new_utxo {
                    <MatCircularProgress indeterminate=true />
                } else {
                    <div class="col-4" onclick={onclick}>
                        <MatButton label="Create new UTXOs" raised=true/>
                    </div>
                }
            </div>
            <p class="message">{(*message).to_string()}</p>
        </>
    }
}

pub struct Page {}

pub enum Msg {}

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <UtxosList/>
            </>
        }
    }
}

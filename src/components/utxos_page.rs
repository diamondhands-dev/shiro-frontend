use material_yew::{MatButton, MatCircularProgress};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, Html, Properties};

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

#[derive(Serialize, Deserialize, PartialEq)]
pub struct UtxosResult {
    created_utxos: u8,
}

#[derive(Properties, PartialEq)]
pub struct UtxosListProps {}

#[function_component(UtxosList)]
pub fn utxo_list(_props: &UtxosListProps) -> Html {
    let new_utxo = use_state(|| false);
    let message = use_state(|| "".to_string());

    let utxo_list = use_state_eq(Vec::<UtxoRGB>::new);

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
                        let res_text = res.text().await.unwrap();
                        match serde_json::from_str::<UtxosResult>(&res_text) {
                            Ok(json) => {
                                message.set(json.created_utxos.to_string());
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                                message.set(res_text);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                }
            });
        })
    };

    let onload = {
        let baseurl = web_sys::window().unwrap().origin();
        let message = message.clone();
        let unspents = UnspentsParams { settled_only: true };
        let client = reqwest::Client::new();
        let u_list = utxo_list.clone();
        spawn_local(async move {
            let res = client
                .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/unspents")
                .json(&unspents)
                .send()
                .await;
            match res {
                Ok(res) => {
                    let res_text = res.text().await.unwrap();
                    match serde_json::from_str::<UnspentsResult>(&res_text) {
                        Ok(json) => {
                            u_list.set(json.unspents);
                        }
                        Err(e) => {
                            log::error!("{:?}", e);
                            message.set(res_text);
                        }
                    }
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
        });
        //let utxo_list = json.unspents;
        (*utxo_list)
            .iter()
            .enumerate()
            .map(|(_, utxo_rgb)| {
                html! {
                    // Passing data `asset_id`
                    <div class="container">
                        <div class="list-group-item list-group-item-action flex-column align-items-start">
                            <div class="d-flex w-100 justify-content-between">
                                <a class="mb-1 truncate" href={"https://mempool.space/testnet/tx/".to_owned() + &utxo_rgb.utxo.outpoint.txid.clone() + "#vout=" + &utxo_rgb.utxo.outpoint.vout.clone().to_string()} target="_blank">{utxo_rgb.utxo.outpoint.txid.clone()}</a>
                                <p class="mb-1">{utxo_rgb.utxo.btc_amount.clone()}{" sat"}</p>
                            </div>
                            <div class="d-flex w-100 justify-content-between">
                                if !utxo_rgb.rgb_allocations.is_empty() {
                                    <div class="mb-1 truncate">{utxo_rgb.rgb_allocations[0].asset_id.clone()}</div>
                                    <div class="mb-1">{utxo_rgb.rgb_allocations[0].amount.clone()}</div>
                                } else {
                                    <div class="mb-1 truncate">{""}</div>
                                    <div class="mb-1">{""}</div>
                                }
                            </div>
                        </div>
                   </div>
                }
            })
            .collect::<Html>()
        //html! {}
    };

    html! {
        <>
            <div>
                if *new_utxo {
                    <MatCircularProgress indeterminate=true />
                } else {
                    <h5 class="mb-2">
                        {"Create new UTXOs for allocation"}
                    </h5>
                    <div class="col-4" onclick={onclick}>
                        <MatButton label="Create" raised=true/>
                    </div>
                }
            </div>
            <p class="message">{(*message).to_string()}</p>
            <div>
                {"UTXO List"}
                {onload}
            </div>
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

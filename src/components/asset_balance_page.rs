use super::asset_list_page::{AssetType, AssetsParams, AssetsResult};
use super::common::RefreshButton;
use super::utxos_page::Outpoint;
use material_yew::MatButton;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, prelude::*, use_state, virtual_dom::AttrValue, Html, Properties,
};
use yew_router::prelude::*;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Transfer {
    idx: String,
    created_at: String,
    updated_at: String,
    status: String,
    amount: String,
    kind: String,
    txid: Option<String>,
    blinded_utxo: Option<String>,
    unblinded_utxo: Option<Outpoint>,
    change_utxo: Option<Outpoint>,
    blinding_secret: Option<String>,
    expiration: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TransfersParams {
    pub asset_id: String,
}

enum PageMode {
    RGB20,
    RGB25,
    Unknown,
}

#[derive(Properties, PartialEq)]
pub struct SendReceiveButtonProp {
    pub asset_id: String,
    pub label: String,
}

#[function_component(ReceiveButton)]
pub fn receive_button(props: &SendReceiveButtonProp) -> Html {
    let navigator = use_history().unwrap();
    let asset_id = props.asset_id.clone();
    let onclick = Callback::from(move |_| {
        let asset_id = asset_id.clone();
        navigator.push(crate::Route::AssetReceive { asset_id });
    });
    html! {
        <div {onclick}>
            <MatButton label={props.label.clone()} icon={AttrValue::from("call_received")} raised=true />
        </div>
    }
}

#[function_component(SendButton)]
fn send_button(props: &SendReceiveButtonProp) -> Html {
    let navigator = use_history().unwrap();
    let asset_id = props.asset_id.clone();
    let onclick = Callback::from(move |_| {
        let asset_id = asset_id.clone();
        navigator.push(crate::Route::AssetSend { asset_id });
    });
    html! {
        <div {onclick}>
            <MatButton label={props.label.clone()} icon={AttrValue::from("arrow_outward")} raised=true />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AssetBalancePageInnerProp {
    asset_id: String,
}

#[function_component(AssetBalancePageInner)]
pub fn asset_balance_page(prop: &AssetBalancePageInnerProp) -> Html {
    let asset_id = prop.asset_id.clone();
    let page_mode = use_state(|| PageMode::Unknown);
    let name = use_state(|| "Unknown".to_string());
    let ticker = use_state(|| "UNKNOWN".to_string());
    let total_balance = use_state(|| 0.0f64);
    let precision = use_state(|| 0i32);

    let transfer_list = use_state_eq(Vec::<Transfer>::new);

    let content = {
        match *page_mode {
            PageMode::Unknown => {
                let baseurl = web_sys::window().unwrap().origin();
                let t_list = transfer_list.clone();
                let client = reqwest::Client::new();
                let name = name.clone();
                let ticker = ticker.clone();
                let total_balance = total_balance.clone();
                let precision = precision;
                spawn_local(async move {
                    let res = client
                        .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/assets")
                        .json(&AssetsParams {
                            filter_asset_types: Vec::<AssetType>::new(),
                        })
                        .send()
                        .await;
                    if let Ok(res) = res {
                        match res.json::<AssetsResult>().await {
                            Ok(json) => {
                                {
                                    let rgb20s = json
                                        .assets
                                        .rgb20
                                        .into_iter()
                                        .filter(|x| x.asset_id == *asset_id)
                                        .collect::<Vec<_>>();
                                    if rgb20s.len() == 1 {
                                        let rgb20 = rgb20s[0].clone();
                                        page_mode.set(PageMode::RGB20);
                                        //asset_id.set(rgb20.asset_id.clone());
                                        name.set(rgb20.name.clone());
                                        ticker.set(rgb20.ticker.clone());
                                        let future = rgb20.balance.future.parse::<f64>().unwrap();
                                        precision.set(rgb20.precision as i32);
                                        let precision = rgb20.precision;
                                        total_balance.set((future) / 10f64.powi(precision as i32));
                                        //return;
                                    }
                                }
                                {
                                    let rgb25s = json
                                        .assets
                                        .rgb25
                                        .into_iter()
                                        .filter(|x| x.asset_id == *asset_id)
                                        .collect::<Vec<_>>();
                                    if rgb25s.len() == 1 {
                                        let rgb25 = rgb25s[0].clone();
                                        page_mode.set(PageMode::RGB25);
                                        //asset_id.set(rgb25.asset_id.clone());
                                        name.set(rgb25.name.clone());
                                        total_balance.set(rgb25.balance.spendable.parse().unwrap());
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                            }
                        }
                    };

                    let params = TransfersParams {
                        asset_id: asset_id.clone(),
                    };
                    let res = client
                        .put(
                            API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned()
                                + "/wallet/transfers",
                        )
                        .json(&params)
                        .send()
                        .await;
                    log::info!("{:#?}", res);
                    match res {
                        Ok(res) => match res.json::<Vec<Transfer>>().await {
                            Ok(json) => {
                                t_list.set(json);
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
            }
            PageMode::RGB20 => {}
            PageMode::RGB25 => {}
        }
        //let now = std::time::Instant::now();
        //log::info!("{:#?}", now);
        html! {
           <>
           {"Transactions"}
           //<div class="list-group">
           {
               (*transfer_list).iter().enumerate().map(|(_,transfer)| {
                   // x conf is needed to be settled, otherwise the status stays `WaitingConfirmations`
                   // FIXME  `transfer.expiration`
                   html! {
                       if !(transfer.status == "WaitingCounterparty" && transfer.expiration.clone()
                           .unwrap_or_else(|| "1".to_string())
                           .parse::<u128>()
                           .unwrap() < 1678775312 || transfer.amount.clone().parse::<f64>().unwrap() == 0.0) {
                           <div class="list-group-item list-group-item-action flex-column align-items-start">
                               <div class="d-flex w-100 justify-content-between">

                                   if transfer.kind == "receive" {
                                       <h5 class="mb-1">{"+"}{transfer.amount.clone()}</h5>

                                   } else if transfer.kind == "send" {
                                       <h5 class="mb-1">{"-"}{transfer.amount.clone()}</h5>

                                   } else if transfer.kind == "issuance" {
                                       <h5 class="mb-1">{"+"}{transfer.amount.clone()}</h5>
                                   }
                                   //<small>{spendable / 10f64.powi(asset.precision as i32)}</small>
                                </div>
                                <div class="d-flex w-100 justify-content-between">
                                    <p class="mb-1">{transfer.status.clone()}</p>
                                    if transfer.txid.clone().unwrap_or_default() != "" {
                                        <a class="mb-1 truncate" href={"https://mempool.space/testnet/tx/".to_owned() + &transfer.txid.clone().unwrap_or_default()} target="_blank">{transfer.txid.clone().unwrap_or_default()}</a>
                                    }

                               </div>
                           </div>
                       }
                   }}).collect::<Html>()
           }
           </>
        }
    };

    html! {
        <>
        <div class="container">
            <div class="text-center">
                <h1>{(*name).clone()}</h1>
                <div>{"Total Balance"}</div>
                <h2>{*total_balance.clone()} {" "} {(*ticker).clone()}</h2>
            </div>
            <div class="row justify-content-evenly">
                <div class="container border asset_id">
                    <div>{"Asset ID"}</div>
                    <div>{prop.asset_id.clone()}</div>
                </div>
                <div class="m-4">
                    <div class="row justify-content-evenly">
                        <div class="col-4">
                            <ReceiveButton label="Receive" asset_id={prop.asset_id.clone()}/>
                        </div>
                        <div class="col-4">
                            <SendButton label="Send  " asset_id={prop.asset_id.clone()}/>
                        </div>
                    </div>
                </div>
            </div>
            //<div class="row col-1">
            //    <h2 class="col">{"Transactions"}</h2>
            //</div>
        </div>
        {content}
        <RefreshButton/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub asset_id: String,
}

pub struct Page;

impl Component for Page {
    type Properties = Props;
    type Message = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <AssetBalancePageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

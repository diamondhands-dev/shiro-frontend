use super::asset_list_page::{AssetType, AssetsParams, AssetsResult};
use material_yew::{MatButton, MatCircularProgress, MatTextField};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Serialize, Deserialize)]
pub struct SendParams {
    recipient_map: HashMap<String, Vec<Recipient>>,
    donation: bool,
    fee_rate: f32,
}

#[derive(Serialize, Deserialize)]
struct Recipient {
    blinded_utxo: String,
    amount: String,
    consignment_endpoints: Vec<String>,
}

enum PageMode {
    RGB20,
    RGB121,
    Unknown,
}
#[derive(Properties, PartialEq)]
pub struct AssetSendPageInnerProp {
    asset_id: String,
}

#[function_component(AssetSendPageInner)]
pub fn asset_send_page(prop: &AssetSendPageInnerProp) -> Html {
    let message = use_state(|| "".to_string());
    let sending = use_state(|| false);

    let asset_id = use_state(|| prop.asset_id.clone());
    let page_mode = use_state(|| PageMode::Unknown);
    let ticker = use_state(|| "UNKNOWN".to_string());
    let total_balance = use_state(|| 0.0f64);
    let spendable_balance = use_state(|| 0.0f64);
    let pay_to = use_state(|| "".to_string());
    let amount_to_pay = use_state(|| 0.0f64);
    let fee_rate = use_state(|| 0.0f32);
    let invalid_form = use_state(|| true);
    let precision = use_state(|| 0i32);

    let onload = {
        match *page_mode {
            PageMode::Unknown => {
                let baseurl = web_sys::window().unwrap().origin();
                let client = reqwest::Client::new();
                let asset_id = asset_id.clone();
                let ticker = ticker.clone();
                let spendable_balance = spendable_balance.clone();
                let total_balance = total_balance.clone();
                let precision = precision.clone();
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
                                        asset_id.set(rgb20.asset_id.clone());
                                        ticker.set(rgb20.ticker.clone());
                                        let spendable =
                                            rgb20.balance.spendable.parse::<f64>().unwrap();
                                        let future = rgb20.balance.future.parse::<f64>().unwrap();
                                        precision.set(rgb20.precision as i32);
                                        let precision = rgb20.precision;
                                        total_balance.set((future) / 10f64.powi(precision as i32));
                                        spendable_balance
                                            .set(spendable / 10f64.powi(precision as i32));
                                        return;
                                    }
                                }
                                {
                                    let rgb121s = json
                                        .assets
                                        .rgb121
                                        .into_iter()
                                        .filter(|x| x.asset_id == *asset_id)
                                        .collect::<Vec<_>>();
                                    if rgb121s.len() == 1 {
                                        let rgb121 = rgb121s[0].clone();
                                        page_mode.set(PageMode::RGB121);
                                        asset_id.set(rgb121.asset_id.clone());
                                        total_balance
                                            .set(rgb121.balance.spendable.parse().unwrap());
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                            }
                        }
                    };
                });
            }
            PageMode::RGB20 => {}
            PageMode::RGB121 => {}
        }
        html! { <></> }
    };

    let onclick = {
        let sending = sending.clone();
        let message = message.clone();
        let asset_id = asset_id;
        let pay_to = pay_to.clone();
        let amount_to_pay = amount_to_pay.clone();
        let fee_rate = fee_rate.clone();
        let precision = precision;
        Callback::from(move |_: MouseEvent| {
            let baseurl = web_sys::window().unwrap().origin();
            let client = reqwest::Client::new();
            let sending = sending.clone();
            let message = message.clone();
            let asset_id = asset_id.clone();
            let pay_to = pay_to.clone();
            let amount_to_pay = amount_to_pay.clone();
            let fee_rate = fee_rate.clone();
            let precision = precision.clone();
            spawn_local(async move {
                sending.set(true);
                let send_params = SendParams {
                    recipient_map: HashMap::from([(
                        (*asset_id).clone(),
                        vec![Recipient {
                            blinded_utxo: (*pay_to).clone(),
                            amount: (*amount_to_pay * 10f64.powi(*precision)).to_string(),
                            consignment_endpoints: vec![
                                "rgbhttpjsonrpc:http://proxy.rgbtools.org/json-rpc".to_string(),
                            ],
                        }],
                    )]),
                    donation: false,
                    fee_rate: *fee_rate,
                };
                let res = client
                    .post(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/send")
                    .json(&send_params)
                    .send()
                    .await;
                sending.set(false);
                match res {
                    Ok(res) => match res.text().await {
                        Ok(json) => {
                            log::info!("1 {:#?}", json);
                            message.set(json);
                        }
                        Err(e) => {
                            log::info!("2 {:?}", e);
                            message.set(e.to_string());
                        }
                    },
                    Err(e) => {
                        log::info!("3 {:?}", e);
                        message.set(e.to_string());
                    }
                }
            });
        })
    };
    let oninput_pay_to = {
        let pay_to = pay_to.clone();
        Callback::from(move |value: String| {
            pay_to.set(value);
        })
    };

    let oninput_amount_to_pay = {
        let amount_to_pay = amount_to_pay.clone();
        let _invalid_form = invalid_form;
        Callback::from(move |value: String| {
            match value.parse::<f64>() {
                Ok(v) => amount_to_pay.set(v),
                Err(v) => log::error!("{:#?}", v),
            };
        })
    };

    let oninput_fee_rate = {
        let fee_rate = fee_rate.clone();
        Callback::from(move |value: String| {
            match value.parse::<f32>() {
                Ok(v) => fee_rate.set(v),
                Err(v) => log::error!("{:#?}", v),
            };
        })
    };

    let button_disabled = {
        false //TODO: implement.
    };

    html! {
        <>
        <div class="container">
            <h1 style="text-align: center">{"Send"}</h1>
            <div style="text-align: center">{"Total Balence"}</div>
            <h2 style="text-align: center">{*total_balance} {" "} {(*ticker).clone()}</h2>
            <div style="text-align: center">{"Spendable Balence"}</div>
            <h2 style="text-align: center">{*spendable_balance} {" "} {(*ticker).clone()}</h2>
        </div>
        <div class="container">
            <h3>{"Pay to"}</h3>
            <MatTextField outlined=true label="blinded UTXO" value={(*pay_to).clone()} oninput={oninput_pay_to}/>
            <h3>{"Amount to pay"}</h3>
            <MatTextField outlined=true label="0" value={(*amount_to_pay).clone().to_string()} oninput={oninput_amount_to_pay}/>
            <h3>{"Fee rate in sat/vbyte"}</h3>
            <MatTextField outlined=true label="1.5" value={(*fee_rate).clone().to_string()} oninput={oninput_fee_rate}/>
        </div>
        <div class="container">
            if *sending {
                <MatCircularProgress indeterminate=true />
            } else {
                <div onclick={onclick}>
                    <MatButton label="SEND" raised=true disabled={button_disabled} />
                </div>
            }
            <p class="message">{(*message).to_string()}</p>
        </div>
        {onload}
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
            <AssetSendPageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

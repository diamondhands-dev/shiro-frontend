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
    transport_endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InvoiceParams {
    invoice_string: String,
}

enum PageMode {
    RGB20,
    RGB25,
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
    let invoice = use_state(|| "".to_string());
    let message2 = use_state(|| "".to_string());
    let decoding = use_state(|| false);

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
                                    let rgb25s = json
                                        .assets
                                        .rgb25
                                        .into_iter()
                                        .filter(|x| x.asset_id == *asset_id)
                                        .collect::<Vec<_>>();
                                    if rgb25s.len() == 1 {
                                        let rgb25 = rgb25s[0].clone();
                                        page_mode.set(PageMode::RGB25);
                                        asset_id.set(rgb25.asset_id.clone());
                                        total_balance
                                            .set(rgb25.balance.spendable.parse().unwrap());
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
            PageMode::RGB25 => {}
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
                            transport_endpoints: vec![
                                "rpc://proxy.rgbtools.org/json-rpc".to_string(),
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

    let onclick2 = {
        let decoding = decoding.clone();
        let message2 = message2.clone();
        let invoice = invoice.clone();
        Callback::from(move |_: MouseEvent| {
            let baseurl = web_sys::window().unwrap().origin();
            let client = reqwest::Client::new();
            let decoding = decoding.clone();
            let message2 = message2.clone();
            let invoice = invoice.clone();
            spawn_local(async move {
                decoding.set(true);
                let invoice_params = InvoiceParams {
                    invoice_string: invoice.to_string(),
                };
                let res = client
                    .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/invoice")
                    .json(&invoice_params)
                    .send()
                    .await;
                decoding.set(false);
                match res {
                    Ok(res) => match res.text().await {
                        Ok(json) => {
                            log::info!("1 {:#?}", json);
                            message2.set(json);
                        }
                        Err(e) => {
                            log::info!("2 {:?}", e);
                            message2.set(e.to_string());
                        }
                    },
                    Err(e) => {
                        log::info!("3 {:?}", e);
                        message2.set(e.to_string());
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

    let oninput_invoice = {
        let invoice = invoice.clone();
        Callback::from(move |value: String| {
            invoice.set(value);
        })
    };

    let button_disabled = {
        false //TODO: implement.
    };

    html! {
        <>
        <div class="container text-center">

            <div>{"Total Balence"}</div>
            <h3>{*total_balance} {" "} {(*ticker).clone()}</h3>
            <div>{"Spendable Balence"}</div>
            <h3>{*spendable_balance} {" "} {(*ticker).clone()}</h3>

        <div>
            <h5>{"Pay to"}</h5>
            <MatTextField outlined=true label="blinded UTXO" value={(*pay_to).clone()} oninput={oninput_pay_to}/>
            <h5>{"Amount to pay"}</h5>
            <MatTextField outlined=true label="0" value={(*amount_to_pay).clone().to_string()} oninput={oninput_amount_to_pay}/>
            <h5>{"Fee rate in sat/vbyte"}</h5>
            <MatTextField outlined=true label="1.5" value={(*fee_rate).clone().to_string()} oninput={oninput_fee_rate}/>
        </div>

        <div class="m-3">
            if *sending {
                <MatCircularProgress indeterminate=true />
            } else {
                <div onclick={onclick}>
                    <MatButton label="SEND" raised=true disabled={button_disabled} />
                </div>
            }
            <p class="message">{(*message).to_string()}</p>
        </div>

        <div class="m-3">
            <p>
                <a class="" data-bs-toggle="collapse" href="#collapseExample" role="button" aria-expanded="false" aria-controls="collapseExample">
                {"Decode Invoice"}
                </a>
            </p>
        </div>
        <div class="collapse" id="collapseExample">
            <div class="card card-body">
                <div>
                    <MatTextField outlined=true label="invoice" value={(*invoice).clone()} oninput={oninput_invoice}/>
                </div>
                <div class="m-3">
                    if *decoding {
                        <MatCircularProgress indeterminate=true />
                    } else {
                        <div onclick={onclick2}>
                            <MatButton label="DECODE" raised=true />
                        </div>
                    }
                    <p class="message">{(*message2).to_string()}</p>
                </div>
            </div>
        </div>


        {onload}

        </div>
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

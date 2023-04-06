use material_yew::{MatCircularProgress, MatTextField};
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Clone, Serialize, Deserialize)]
pub struct BlindParams {
    asset_id: Option<String>,
    amount: Option<String>,
    duration_seconds: Option<u32>,
    consignment_endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BlindData {
    invoice: String,
    blinded_utxo: String,
    blinding_secret: String,
    expiration_timestamp: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct QrCodeProp {
    invoice: String,
}

#[function_component(QrCodeView)]
pub fn qr_code(props: &QrCodeProp) -> Html {
    let code =
        QrCode::with_version((props.invoice).as_bytes(), Version::Normal(12), EcLevel::M).unwrap();
    let image = code
        .render()
        .min_dimensions(300, 300)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&image);
    Html::VRef(div.into())
}

#[derive(Properties, PartialEq)]
pub struct AssetReceivePageInnerProp {
    asset_id: String,
}

#[function_component(AssetReceivePageInner)]
pub fn asset_receive_page(props: &AssetReceivePageInnerProp) -> Html {
    let baseurl = web_sys::window().unwrap().origin();
    let message = use_state(|| "".to_string());
    let invoice = use_state(|| "".to_string());
    let blinded_utxo = use_state(|| "".to_string());
    let counter = use_state(|| 0);
    let loading = use_state(|| false);

    let _onload = {
        let loading = loading.clone();
        let message = message.clone();
        let invoice = invoice.clone();
        let blinded_utxo = blinded_utxo.clone();
        let asset_id = if props.asset_id.starts_with('_') {
            None
        } else {
            Some(props.asset_id.clone())
        };
        if *counter == 0 {
            spawn_local(async move {
                counter.set(*counter + 1);
                loading.set(true);
                let blind_params = BlindParams {
                    asset_id,
                    amount: None,
                    duration_seconds: Some(3600), // 1hour
                    consignment_endpoints: vec![
                        "rgbhttpjsonrpc:http://proxy.rgbtools.org/json-rpc".to_string(),
                    ],
                };
                let client = reqwest::Client::new();
                let res = client
                    .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/blind")
                    .json(&blind_params)
                    .send()
                    .await;
                match res {
                    Ok(res) => {
                        let res_text = res.text().await.unwrap();
                        match serde_json::from_str::<BlindData>(&res_text) {
                            Ok(json) => {
                                invoice.set(json.invoice);
                                blinded_utxo.set(json.blinded_utxo);
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                                message.set(res_text);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                        message.set(e.to_string());
                    }
                }
                log::info!("counter {:?}", counter);
                loading.set(false);
            });
            //html! { <></> }
        }
    };

    html! {
        <>
        <div class="container text-center">

            if !*loading && message.is_empty() {
                <QrCodeView invoice={(*invoice).clone()} />
                <div id="qrcode"/>
                <div class="d-flex flex-column bd-highlight mb-3">
                    <div class="p-2">
                        <MatTextField outlined=true label="invoice" value={(*invoice).clone()}/>
                    </div>
                    <div class="p-2">
                        <MatTextField outlined=true label="blinded UTXO" value={(*blinded_utxo).clone()}/>
                    </div>
                </div>
                <div>{"The blinded UTXO in this invoice will expire in 1 hours after its creation and will be valid only for this asset"}</div>
                //{onload}
            } else if !message.is_empty() {
                <p class="message">{(*message).to_string()}</p>
            } else {
                <div>
                    <MatCircularProgress indeterminate=true />
                </div>
            }
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
            <AssetReceivePageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

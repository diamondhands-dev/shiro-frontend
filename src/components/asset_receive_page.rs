use material_yew::{MatButton, MatTextField};
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
        QrCode::with_version((*&props.invoice).as_bytes(), Version::Normal(12), EcLevel::M).unwrap();
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
    let invoice = use_state(|| "".to_string());
    let blinded_utxo = use_state(|| "".to_string());
    // FIXME: Multiple HTTP requests are called due to the state change by use_state.
    let count = use_mut_ref(|| 0);

    let onload = {
        let invoice = invoice.clone();
        let blinded_utxo = blinded_utxo.clone();
        let asset_id = if props.asset_id.starts_with('_') {
            None
        } else {
            Some(props.asset_id.clone())
        };
        spawn_local(async move {
            let blind_params = BlindParams {
                asset_id,
                amount: None,
                duration_seconds: Some(3600), // 1hour
                consignment_endpoints: vec![
                    "rgbhttpjsonrpc:http://proxy.rgbtools.org/json-rpc".to_string()
                ],
            };
            let client = reqwest::Client::new();
            let res = client
                //.put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/blind")
                .put(API_ROOT.unwrap_or("http://localhost:8080").to_owned() + "/wallet/blind")
                .json(&blind_params)
                .send()
                .await;
            match res {
                Ok(res) => match res.json::<BlindData>().await {
                    Ok(json) => {
                        log::info!("1: {:#?}", (*count.borrow_mut()).to_string());
                        *count.borrow_mut() += 1;
                        if *count.borrow_mut() < 2 {
                            invoice.set(json.invoice);
                            blinded_utxo.set(json.blinded_utxo);
                        } else {

                        }
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
        html! { <></> }
    };

    //let onclick = Callback::from(|_| {});
    html! {
        <>
        <div class="container">
            <h1 style="text-align: center">{"Receive"}</h1>
            <QrCodeView invoice={(*invoice).clone()} />
            <div style="text-align: center" id="qrcode"/>
        </div>
        <div class="container">
            <h3>{"Invoice"}</h3>
            <MatTextField outlined=true label="blinded UTXO" value={(*blinded_utxo).clone()}/>
            <div>{"The blinded UTXO in this invoice will expire in 1 hours after its creation and will be valid only for this asset"}</div>
        </div>
        /*
        <div class="container">
            <div onclick={onclick}>
                <MatButton label="COPY" raised=true />
            </div>
        </div>
        */
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
            <AssetReceivePageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

use material_yew::{MatButton, MatTextField};
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

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
        QrCode::with_version((*&props.invoice).as_bytes(), Version::Normal(5), EcLevel::M).unwrap();
    let image = code
        .render()
        .min_dimensions(480, 480)
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

    let onload = {
        let invoice = invoice.clone();
        let asset_id = if props.asset_id.starts_with('_') {
            None
        } else {
            Some(props.asset_id.clone())
        };
        spawn_local(async move {
            let blind_params = BlindParams {
                asset_id,
                amount: None,
                duration_seconds: Some(86400), // 24hours
                consignment_endpoints: vec![
                    "rgbhttpjsonrpc:http://proxy.rgbtools.org/json-rpc".to_string()
                ],
            };
            let client = reqwest::Client::new();
            let res = client
                .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/blind")
                .json(&blind_params)
                .send()
                .await;
            match res {
                Ok(res) => match res.json::<BlindData>().await {
                    Ok(json) => {
                        invoice.set(json.invoice);
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

    let onclick = Callback::from(|_| {});
    html! {
        <>
        <div class="container">
            <h1 style="text-align: center">{"Receive"}</h1>
            <QrCodeView invoice={(*invoice).clone()} />
            <div style="text-align: center" id="qrcode"/>
        </div>
        <div class="container">
            <h3>{"Invoice"}</h3>
            <MatTextField outlined=true label="blinded UTXO" value={(*invoice).clone()}/>
            <div>{"The blinded UTXO in this invoice will expire in 24 hours after its creation and will be valid only for this asset"}</div>
        </div>
        <div class="container">
            <div onclick={onclick}>
                <MatButton label="COPY" raised=true />
            </div>
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
            <AssetReceivePageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

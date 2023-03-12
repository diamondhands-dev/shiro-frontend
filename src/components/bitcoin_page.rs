use material_yew::MatButton;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::virtual_dom::AttrValue;
use wasm_bindgen_futures::spawn_local;
use material_yew::{MatTextField};

#[derive(Serialize, Deserialize)]
pub struct AddressData {
    new_address: String,
}

#[derive(Properties, PartialEq)]
pub struct QrCodeProp {
    address: String,
}

// FIXME: QrCodeView should be abstracted
#[function_component(QrCodeView)]
pub fn qr_code(props: &QrCodeProp) -> Html {
    let code =
        QrCode::with_version((*&props.address).as_bytes(), Version::Normal(5), EcLevel::M).unwrap();
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
pub struct BtcBalancePanelProps {}

#[function_component(BtcBalancePanel)]
pub fn btc_balance_panel(_props: &BtcBalancePanelProps) -> Html {
    let message = use_state(|| "".to_string());
    let address = use_state(|| "".to_string());
    let new_address = use_state(|| false);

    let onclick = {
        let message = message.clone();
        let address = address.clone();
        let new_address = new_address.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();
            let client = reqwest::Client::new();
            let address = address.clone();
            let new_address = new_address.clone();
            spawn_local(async move {
                let res = client
                    .get("http://localhost:8080/wallet/address")
                    .send()
                    .await;
                match res {
                    Ok(res) => {
                        log::info!("{:#?}", res);
                        match res.json::<AddressData>().await {
                            Ok(json) => {
                                address.set(json.new_address);
                                new_address.set(true);
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


    html! {
        <div class="container">
            <h1 style="text-align: center">{"Bitcoin"}</h1>
            <div style="text-align: center">{"Your Balance"}</div>
            <h2 style="text-align: center">{"0 SAT"}</h2>
            <div class="row justify-content-evenly">
                <div class="col-4">
                    <MatButton label="Send" icon={AttrValue::from("code")} raised=true/>
                </div>
                <div class="col-4" onclick={onclick}>
                    <MatButton label="Receive" icon={AttrValue::from("code")} raised=true/>
                </div>
            </div>
            <div class="row col-1">
                <h2 class="col">{"Transactions"}</h2>
            </div>
            <div>
                if *new_address {
                    <QrCodeView address={(*address).clone()} />
                    <MatTextField outlined=true label="Bitcoin address" value={(*address).clone()}/>
                }
            </div>

            <p class="message">{(*message).to_string()}</p>
        </div>
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
            <BtcBalancePanel/>
        }
    }
}

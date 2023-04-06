use material_yew::MatButton;
use material_yew::MatTextField;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

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
        QrCode::with_version((props.address).as_bytes(), Version::Normal(5), EcLevel::M).unwrap();
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
            let baseurl = web_sys::window().unwrap().origin();
            let message = message.clone();
            let client = reqwest::Client::new();
            let address = address.clone();
            let new_address = new_address.clone();
            spawn_local(async move {
                let res = client
                    .get(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/address")
                    .send()
                    .await;
                match res {
                    Ok(res) => {
                        log::info!("{:#?}", res);
                        let res_text = res.text().await.unwrap();
                        match serde_json::from_str::<AddressData>(&res_text) {
                            Ok(json) => {
                                address.set(json.new_address);
                                new_address.set(true);
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

    html! {
        <div class="container text-center">
            <h1>{"Bitcoin"}</h1>
            //<div>{"Your Balance"}</div>
            //<h2>{"0 SAT"}</h2>
            <div class="row justify-content-evenly">
                <div class="col-4">
                    <MatButton label="Send" icon={AttrValue::from("code")} raised=true disabled=true />
                </div>
                <div class="col-4" onclick={onclick}>
                    <MatButton label="Receive" icon={AttrValue::from("code")} raised=true/>
                </div>

                //<div class="row col-1">
                //    <h2 class="col">{"Transactions"}</h2>
                //</div>

                if *new_address {
                    <QrCodeView address={(*address).clone()} />
                    <MatTextField outlined=true label="Bitcoin address" value={(*address).clone()}/>
                }
            </div>

            <p class="message">{(*message).to_string()}</p>

            <div>
            <a class="mb-1" href="https://bitcoinfaucet.uo1.net/" target="_blank">{"Bitcoin faucet"}</a>
            </div>

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

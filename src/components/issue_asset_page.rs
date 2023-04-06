use material_yew::{MatButton, MatCircularProgress, MatFormfield, MatTextField, text_inputs::TextFieldType};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Serialize, Deserialize)]
pub struct IssueParams {
    ticker: String,
    name: String,
    presision: u8,
    amounts: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct IssueAssetsButtonProps {
    onclick: Callback<MouseEvent>,
}

#[function_component(IssueAssetsButton)]
pub fn issue_assets_button(props: &IssueAssetsButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
        })
    };

    html! {
        <div onclick={onclick}>
            <MatButton label="Issue Assets" raised=true />
        </div>
    }
}

#[derive(Properties, PartialEq)]

pub struct IssueAssetPageProps {}

#[function_component(IssueAssetPageInner)]
pub fn page(_props: &IssueAssetPageProps) -> Html {
    let message = use_state(|| "".to_string());
    let new_issue = use_state(|| false);
    let ticker = use_state(|| "".to_string());
    let name = use_state(|| "".to_string());
    let amount = use_state(|| "".to_string());

    let presision = 0;
    let amounts = vec![amount.to_string()];

    let oninput_ticker = {
        let oninput_ticker = ticker.clone();
        Callback::from(move |value: String| {
            oninput_ticker.set(value);
        })
    };
    let oninput_name = {
        let oninput_name = name.clone();
        Callback::from(move |value: String| {
            oninput_name.set(value);
        })
    };
    let oninput_amount = {
        let oninput_amount = amount.clone();
        Callback::from(move |value: String| {
            oninput_amount.set(value);
        })
    };

    let onclick_issue_assets_button = {
        let new_issue = new_issue.clone();
        let message = message.clone();

        let ticker = ticker.clone();
        let name = name.clone();
        Callback::from(move |_| {
            let baseurl = web_sys::window().unwrap().origin();
            let new_issue = new_issue.clone();
            let message = message.clone();

            let ticker = ticker.clone();
            let name = name.clone();
            let presision = presision;
            let amounts = amounts.clone().iter().map(|s| s.to_string()).collect();

            let asset = IssueParams {
                ticker: ticker.to_string(),
                name: name.to_string(),
                presision,
                amounts,
            };
            let client = reqwest::Client::new();
            spawn_local(async move {
                new_issue.set(true);
                let res = client
                    .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/issue/rgb20")
                    .json(&asset)
                    .send()
                    .await;
                log::info!("0 {:#?}", res);
                new_issue.set(false);
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

    html! {
        <div class="container text-center">
            <h3>{"Asset ticker"}</h3>
            <MatTextField outlined=true label="short identifier" value={(*ticker).clone()} oninput={oninput_ticker}/>
            <h3>{"Asset name"}</h3>
            <MatTextField outlined=true label="name of the asset" value={(*name).clone()} oninput={oninput_name}/>
            <h3>{"Total supply"}</h3>
            <MatTextField outlined=true label="amount to issue" field_type={TextFieldType::Number} value={(*amount).clone()} oninput={oninput_amount}/>

            <div class="row m-4">
                <div class="col-12">
                if *new_issue {
                    <MatCircularProgress indeterminate=true />
                } else {
                    <IssueAssetsButton onclick={onclick_issue_assets_button}/>
                }
                </div>
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
            <IssueAssetPageInner/>
        }
    }
}

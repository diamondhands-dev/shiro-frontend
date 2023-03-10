use material_yew::{MatButton, MatTextField};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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
pub fn page(props: &IssueAssetPageProps) -> Html {
    
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
        let ticker_ = ticker.clone();
        let name_ = name.clone();
        Callback::from(move |_| {
            let ticker = ticker_.clone();
            let name = name_.clone();
            let presision = presision;
            let amounts = amounts.clone().iter().map(|s| s.to_string()).collect();
            
            let asset = IssueParams {
                ticker: ticker.to_string(),
                name: name.to_string(),
                presision: presision,
                amounts: amounts,
            };
            let client = reqwest::Client::new();
            spawn_local(async move {
                let res = client
                    //.put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/issue/rgb20")
                    .put("http://127.0.0.1:8080/wallet/issue/rgb20")
                    .json(&asset)
                    .send()
                    .await;
                log::info!("{:#?}", res);
            });
        })
    };


    html! {
        <div class="container">

            <h3>{"Asset ticker"}</h3>
            <MatTextField outlined=true label="short identifier" value={(*ticker).clone()} oninput={oninput_ticker}/>
            <h3>{"Asset name"}</h3>
            <MatTextField outlined=true label="name of the asset" value={(*name).clone()} oninput={oninput_name}/>
            <h3>{"Total supply"}</h3>
            <MatTextField outlined=true label="amount to issue" value={(*amount).clone()} oninput={oninput_amount}/>

            <div class="row">
                <div class="col-12">
                  <IssueAssetsButton onclick={onclick_issue_assets_button}/>
                </div>
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
            <IssueAssetPageInner/>
        }
    }
}

use super::balance_page::{AssetType, AssetsParams, AssetsResult};
use material_yew::MatButton;
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, prelude::*, use_state, virtual_dom::AttrValue, Html, Properties,
};

enum PageMode {
    RGB20,
    RGB121,
    UNKNOWN,
}
#[derive(Properties, PartialEq)]
pub struct AssetBalancePageInnerProp {
    asset_id: String,
}

#[function_component(AssetBalancePageInner)]
pub fn asset_balance_page(prop: &AssetBalancePageInnerProp) -> Html {
    let asset_id = use_state(|| "unknown".to_string());
    let page_mode = use_state(|| PageMode::UNKNOWN);
    let name = use_state(|| "Unknown".to_string());
    let ticker = use_state(|| "UNKNOWN".to_string());
    let total_balance = use_state(|| 0.0f64);

    let onload = {
        log::info!("hogehoge");
        match *page_mode {
            PageMode::UNKNOWN => {
                page_mode.set(PageMode::RGB20);
                asset_id.set("gugahoge".to_string());
                log::info!("hoge");
            }
            _ => {
                log::info!("hoge2");
            }
        };
        match *page_mode {
            PageMode::UNKNOWN => {
                log::info!("hoge5");
                let client = reqwest::Client::new();
                let name = name.clone();
                let ticker = ticker.clone();
                let total_balance = total_balance.clone();
                spawn_local(async move {
                    log::info!("hoge1");
                    let res = client
                        .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/assets")
                        .json(&AssetsParams {
                            filter_asset_types: Vec::<AssetType>::new(),
                        })
                        .send()
                        .await;
                    match res {
                        Ok(res) => match res.json::<AssetsResult>().await {
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
                                        name.set(rgb20.name.clone());
                                        ticker.set(rgb20.ticker.clone());
                                        total_balance.set(rgb20.balance.spendable.parse().unwrap());
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
                                        name.set(rgb121.name.clone());
                                        total_balance
                                            .set(rgb121.balance.spendable.parse().unwrap());
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                            }
                        },
                        _ => {}
                    };
                });
            }
            _ => {}
        }
        html! { <></> }
    };

    html! {
        <>
        <div class="container">
            <h1 style="text-align: center">{(*name).clone()}</h1>
            <div style="text-align: center">{"Total Balance"}</div>
            <h2 style="text-align: center">{*total_balance.clone()} {" "} {(*ticker).clone()}</h2>
            <div class="row justify-content-evenly">
                <div class="container border">
                    <div class="col-12">{"Asset ID"}</div>
                    <div class="col-12">{prop.asset_id.clone()}</div>
                </div>
                <div class="container">
                    <div class="row justify-content-evenly">
                    <div class="col-4">
                        <MatButton label="Receive" icon={AttrValue::from("call_received")} raised=true/>
                    </div>
                    <div class="col-4">
                        <MatButton label="Send" icon={AttrValue::from("arrow_outward")} raised=true/>
                        </div>
                    </div>
                </div>
            </div>
            <div class="row col-1">
                <h2 class="col">{"Transactions"}</h2>
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
            <AssetBalancePageInner asset_id={ctx.props().asset_id.clone()}/>
        }
    }
}

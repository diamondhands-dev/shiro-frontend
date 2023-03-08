use material_yew::MatButton;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct BtcBalancePanelProps {}

#[function_component(BtcBalancePanel)]
pub fn btc_balance_panel(_props: &BtcBalancePanelProps) -> Html {
    html! {
        <div class="container">
            <h1 style="text-align: center">{"bitcoin"}</h1>
            <div style="text-align: center">{"Your Balance"}</div>
            <h2 style="text-align: center">{"78354 SAT"}</h2>
            <div class="row justify-content-evenly">
                <div class="col-4">
                    <MatButton label="Send" icon={AttrValue::from("code")} raised=true/>
                </div>
                <div class="col-4">
                    <MatButton label="Receive" icon={AttrValue::from("code")} raised=true/>
                </div>
            </div>
            <div class="row col-1">
                <h2 class="col">{"Transactions"}</h2>
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

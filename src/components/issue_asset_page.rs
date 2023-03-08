use material_yew::{MatButton, MatTextField};
use yew::prelude::*;

#[derive(Properties, PartialEq)]

pub struct IssueAssetPageProps {}

#[function_component(IssueAssetPageInner)]
pub fn page(props: &IssueAssetPageProps) -> Html {
    html! {
        <div class="container">
            <h1 style="text-align: center">{"Issue new asset"}</h1>
            <div class="container">
                <div class="row">
                    <div class="col-12">
                        <h2>{"Asset ticker"}</h2>
                    </div>
                    <div class="col-12">
                        <MatTextField outlined=true label={"short identifier:"} />
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="row">
                    <div class="col-12">
                        <h2>{"name of the asset"}</h2>
                    </div>
                    <div class="col-12">
                        <MatTextField outlined=true label={"short identifier"} />
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="row">
                    <div class="col-12">
                        <h2>{"Total supploy"}</h2>
                    </div>
                    <div class="col-12">
                        <MatTextField outlined=true label={"amount to issue"} />
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="row">
                    <div class="col-12">
                        <MatButton outlined=true label={"ISSUE ASSET"} />
                    </div>
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

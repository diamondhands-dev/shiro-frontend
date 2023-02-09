use ::material_yew::{MatTab, MatTabBar};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BalancePageProps {}

#[function_component(BalancePageInner)]
pub fn page(_props: &BalancePageProps) -> Html {
    html! {
        <>
             <MatTabBar>
                 <MatTab icon="wallet" label="Fungible">{"hoge"}</MatTab>
                 <MatTab icon="image" label="NFT" />
             </MatTabBar>
        </>
    }
}

pub struct Page {}

impl Component for Page {
    type Properties = ();
    type Message = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <BalancePageInner/>
        }
    }
}

use super::fungible_list::List as FungibleList;
use material_yew::{MatTab, MatTabBar};
use yew::prelude::*;

pub struct BalanceTab;

pub enum BalanceTabMsg {}

impl Component for BalanceTab {
    type Message = BalanceTabMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
        <section>
            <MatTabBar>
                <MatTab icon="wallet" label="Fungible"/>
                <MatTab icon="image" label="NFT" />
            </MatTabBar>
            <FungibleList/>
        </section>
        }
    }
}

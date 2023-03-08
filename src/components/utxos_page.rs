use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UtxosListProps {}

#[function_component(UtxosList)]
pub fn utxo_list(_props: &UtxosListProps) -> Html {
    html! {
        <></>
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
            <>
                <UtxosList/>
            </>
        }
    }
}

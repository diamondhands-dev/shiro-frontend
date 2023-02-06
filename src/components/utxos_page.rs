use super::utxos_list::List as UtxosList;
use yew::prelude::*;

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
            <section>
                <UtxosList/>
            </section>
        }
    }
}

use super::mnemonic_list::List as MnemonicList;
use yew::prelude::*;

pub struct Page {
}

pub enum Msg {
}

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section>
                <MnemonicList/>
            </section>
        }
    }
}

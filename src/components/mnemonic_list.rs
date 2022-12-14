use material_yew::{list::ListIndex, MatList, MatListItem, MatTextField, WeakComponentLink};
use yew::prelude::*;

pub struct List {}

pub enum Msg {}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                <MatList>
                    <MatListItem><MatTextField outlined=true label="1st word" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="2nd" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="3rd" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="4th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="5th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="6th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="7th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="8th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="9th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="10th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="11th" /></MatListItem>
                    <MatListItem><MatTextField outlined=true label="12th" /></MatListItem>
                </MatList>
            </div>
        }
    }
}

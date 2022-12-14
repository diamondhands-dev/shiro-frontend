use material_yew::{list::ListIndex, MatList, MatListItem, WeakComponentLink};
use yew::prelude::*;

pub struct List {
    selected_index: usize,
    list_link: WeakComponentLink<MatList>,
}

pub enum Msg {
    Action(ListIndex),
    Focus,
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_index: 0,
            list_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Action(val) => {
                self.selected_index = match val {
                    ListIndex::Single(val) => match val {
                        Some(val) => val,
                        None => return false,
                    },
                    _ => return false,
                };
                true
            }
            Msg::Focus => {
                self.list_link.focus_item_at_index(0);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
                    <section>
            <MatList onaction={link.callback(|val| Msg::Action(val))} list_link={self.list_link.clone()}>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>

            <span>{"Selected index: "}{&self.selected_index}</span>
        </section>
                }
    }
}

use material_yew::{list::ListIndex, MatList, MatListItem, WeakComponentLink};
use yew::prelude::*;

pub struct List {
    selected_index: usize,
    list_link: WeakComponentLink<MatList>,
}

pub enum Msg {
    Action(ListIndex),
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
                    <section>
            <MatList onaction={link.callback(|val| Msg::Action(val))} list_link={self.list_link.clone()}>
                <MatListItem>{"8bb9917d0386d8fd57699509866d4c125b07c08f6e5af812d849f31c3961c44c:0"}</MatListItem>
                <MatListItem>{"8bb9917d0386d8fd57699509866d4c125b07c08f6e5af812d849f31c3961c44c:1"}</MatListItem>
                <MatListItem>{"584ea0356549984290bef087ffcb7b3a0a4aa0ad2a1e0624fbea39275a6d75e3:0"}</MatListItem>
                <MatListItem>{"f7c36938999e22484955cb1363763bf522f617c6e85be10f9dc71b04e4e38674:3"}</MatListItem>
            </MatList>

            <span>{"Selected index: "}{&self.selected_index}</span>
        </section>
                }
    }
}

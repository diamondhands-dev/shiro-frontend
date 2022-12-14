use super::mnemonic_list::List as MnemonicList;
use bip39::{Language, Mnemonic};
use gloo::storage::{LocalStorage, Storage};
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

const KEY: &'static str = "shiro.mnemonic";

pub struct Page {
    mnemonic: String,
}

pub enum Msg {
    GenerateKeys,
}

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            mnemonic: LocalStorage::get(KEY).unwrap_or_else(|_| { LocalStorage::set(KEY, "").ok(); "".to_string() }),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        let m = Mnemonic::generate_in_with(&mut rand::thread_rng(), Language::English, 12).unwrap();
        println!("{}", m);
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

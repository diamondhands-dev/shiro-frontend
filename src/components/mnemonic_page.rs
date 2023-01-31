use bip39::{Language, Mnemonic};
use gloo::storage::{LocalStorage, Storage};
use material_yew::{list::ListIndex, MatList, MatListItem, MatTextField, WeakComponentLink};
use serde_derive::{Deserialize, Serialize};
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const KEY: &'static str = "shiro.mnemonic";

#[derive(Clone, PartialEq)]
pub struct MnemonicWord {
    pub word: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct MnemonicWordProp {
    pub word: String,
    pub label: String,
}

#[function_component(MnemonicWordField)]
fn word_field(props: &MnemonicWordProp) -> Html {
    let word = use_state(|| "".to_string());
    let oninput = {
        let word = word.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                Some(value) => {
                    word.set((*word).clone() + &value);
                }
                None => {
                    word.set("".to_string());
                }
            }
        })
    };

    html! {
        <MatListItem><MatTextField outlined=true label={props.label.clone()} value={(*word).clone()} /></MatListItem>
    }
}

#[function_component(MnemonicWordList)]
pub fn mnemonic_word_list() -> Html {
    let mut word_list = Vec::<MnemonicWord>::new();
    for i in 1..13 {
        word_list.push(MnemonicWord {
            label: i.to_string(),
            word: "".to_string(),
        });
    }
    html! {
        <div>
        <MatList>
        { word_list.iter().map(|mnemonic_word| html! {
            <MnemonicWordField label={mnemonic_word.label.clone()} word={mnemonic_word.word.clone()} />
                                                   }).collect::<Html>()
        }
        </MatList>
            </div>
    }
}

#[function_component(GenerateKeysButton)]
pub fn generate_keys_button() -> Html {
    let generate_onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let m = Mnemonic::generate_in(Language::English, 12).unwrap();
            log::info!("{}", m);
        })
    };

    html! {
        <button class="btn btn-primary" onclick={generate_onclick} >{"Generate Keys"}</button>
    }
}

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
            mnemonic: LocalStorage::get(KEY).unwrap_or_else(|_| {
                LocalStorage::set(KEY, "").ok();
                "".to_string()
            }),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        let m = Mnemonic::generate_in(Language::English, 12).unwrap();
        println!("{}", m);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section>
                <h1>{"Fill your mnemonic word in."}</h1>
                <MnemonicWordList/>
                <GenerateKeysButton/>
            </section>
        }
    }
}

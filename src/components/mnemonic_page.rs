use bip39::{Language, Mnemonic};
use gloo::storage::{LocalStorage, Storage};
use material_yew::{
    list::ListIndex, MatButton, MatList, MatListItem, MatTextField, WeakComponentLink,
};
use serde_derive::{Deserialize, Serialize};
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const KEY: &'static str = "shiro.mnemonic";

pub struct MnemonicWord {
    pub idx: isize,
    pub word: String,
}

#[derive(Properties, PartialEq)]
pub struct MnemonicWordProp {
    pub label: String,
    pub word: String,
    pub oninput: Callback<String>,
}

#[function_component(MnemonicWordField)]
pub fn mnemonic_word_field(props: &MnemonicWordProp) -> Html {
    html! {
        <MatListItem>
            <MatTextField outlined=true label={props.label.clone()} oninput={props.oninput.clone()} />
        </MatListItem>
    }
}

#[derive(Properties, PartialEq)]
pub struct MnemonicWordListProp {
    mnemonic: String,
    onchanged: Callback<String>,
}

#[function_component(MnemonicWordList)]
pub fn mnemonic_word_list(props: &MnemonicWordListProp) -> Html {
    let mut word_list = Vec::<MnemonicWord>::new();
    for i in 0..12 {
        word_list.push(MnemonicWord {
            idx: i,
            word: "".to_string(),
        });
    }

    html! {
        <>
        { word_list.iter().map(|word| {
            let oninput = {
                let onchanged = props.onchanged.clone();
                Callback::from(move |message: String| {
                    onchanged.emit(message.clone());
                })
            };
            html! {
                <MnemonicWordField label={(word.idx + 1).to_string()} word={word.word.clone()} {oninput} />
            }}).collect::<Html>()
        }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct GenerateKeysButtonProps {
    onclick: Callback<String>,
}

#[function_component(GenerateKeysButton)]
pub fn generate_keys_button(props: &GenerateKeysButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit("".to_string());
        })
    };

    html! {
        <div {onclick}>
            <MatButton label="Generate Keys" outlined=true />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RevertFormButtonProps {
    onclick: Callback<String>,
}

#[function_component(RevertFormButton)]
pub fn revert_form_button(props: &RevertFormButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit("".to_string());
        })
    };

    html! {
        <div onclick={onclick}>
            <MatButton label="revert" outlined=true/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ClearFormButtonProps {
    onclick: Callback<String>,
}

#[function_component(ClearFormButton)]
pub fn clear_form_button(props: &ClearFormButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit("".to_string());
        })
    };

    html! {
        <div onclick={onclick}>
            <MatButton label="Clear" outlined=true/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct OpenWalletButtonProps {
    onclick: Callback<String>,
    disabled: bool,
}

#[function_component(OpenWalletButton)]
pub fn open_wallet_button(props: &OpenWalletButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit("".to_string());
        })
    };

    html! {
        <div onclick={onclick}>
            <MatButton label="Open Your Wallet" raised=true disabled={props.disabled} />
        </div>
    }
}

pub struct Page {
    mnemonic: String,
    unsaved_mnemonic: String,
    is_invalid_mnemonic: bool,
}

#[derive(Properties, PartialEq)]
pub struct PageProperties {}

pub enum Msg {
    ClearForm,
    GenerateKeys,
    OpenWallet,
    RevertForm,
}

impl Component for Page {
    type Message = Msg;
    type Properties = PageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let mnemonic = LocalStorage::get(KEY).unwrap_or_else(|_| {
            LocalStorage::set(KEY, "").ok();
            "".to_string()
        });
        Self {
            unsaved_mnemonic: mnemonic.clone(),
            mnemonic,
            is_invalid_mnemonic: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClearForm => self.mnemonic = "".to_string(),
            Msg::GenerateKeys => {
                self.mnemonic = Mnemonic::generate_in(Language::English, 12)
                    .unwrap()
                    .to_string()
            }
            Msg::OpenWallet => {
                LocalStorage::set(KEY, self.mnemonic.clone()).unwrap();
            }
            Msg::RevertForm => self.mnemonic = self.unsaved_mnemonic.clone(),
        }
        log::info!("update! {}", self.mnemonic);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchanged = {
            Callback::from(move |message: String| {
                log::info!("{}", message);
            })
        };
        html! {
            <section>
                <h1>{"Fill your mnemonic word in."}</h1>
                <MatList>
                    <MnemonicWordList mnemonic={self.mnemonic.clone()} {onchanged}/>
                </MatList>
                <GenerateKeysButton onclick={link.callback(|_|Msg::GenerateKeys)}/>
                <RevertFormButton onclick={link.callback(|_|Msg::RevertForm)}/>
                <ClearFormButton onclick={link.callback(|_|Msg::ClearForm)}/>
                <OpenWalletButton onclick={link.callback(|_|Msg::OpenWallet)} disabled={self.is_invalid_mnemonic}/>
            </section>
        }
    }
}

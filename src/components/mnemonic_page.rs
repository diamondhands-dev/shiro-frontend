use bdk::bitcoin::{secp256k1::Secp256k1, Network};
use bdk::keys::bip39::{Language, Mnemonic};
use bdk::keys::{DerivableKey, ExtendedKey};
use gloo::storage::{LocalStorage, Storage};
use material_yew::{MatButton, MatList, MatListItem, MatTextField};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const KEY: &'static str = "shiro.mnemonic";

#[derive(Properties, PartialEq)]
pub struct MnemonicWordProp {
    pub label: String,
    pub value: String,
    pub oninput: Callback<String>,
}

#[function_component(MnemonicWordField)]
pub fn mnemonic_word_field(props: &MnemonicWordProp) -> Html {
    html! {
        <MatListItem>
            <MatTextField outlined=true label={props.label.clone()} value={props.value.clone()} oninput={props.oninput.clone()} />
        </MatListItem>
    }
}

#[derive(Properties, PartialEq)]
pub struct MnemonicWordListProp {
    words: Vec<String>,
    onchanged: Callback<String>,
}

#[function_component(MnemonicWordList)]
pub fn mnemonic_word_list(props: &MnemonicWordListProp) -> Html {
    html! {
        <>
        { props.words.iter().enumerate().map(|(idx, word)| {
            let oninput = {
                let onchanged = props.onchanged.clone();
                Callback::from(move |message: String| {
                    onchanged.emit(message.clone());
                })
            };
            html! {
                <MnemonicWordField label={(idx + 1).to_string()} value={word.clone()} {oninput}/>
            }}).collect::<Html>()
        }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct GenerateKeysButtonProps {
    onclick: Callback<MouseEvent>,
}

#[function_component(GenerateKeysButton)]
pub fn generate_keys_button(props: &GenerateKeysButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
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
    onclick: Callback<MouseEvent>,
}

#[function_component(RevertFormButton)]
pub fn revert_form_button(props: &RevertFormButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
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
    onclick: Callback<MouseEvent>,
}

#[function_component(ClearFormButton)]
pub fn clear_form_button(props: &ClearFormButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
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
    onclick: Callback<MouseEvent>,
    disabled: bool,
}

#[function_component(OpenWalletButton)]
pub fn open_wallet_button(props: &OpenWalletButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
        })
    };

    html! {
        <div onclick={onclick}>
            <MatButton label="Open Your Wallet" raised=true disabled={props.disabled} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PageProps {}

#[function_component(MnemonicPageInner)]
pub fn page(_: &PageProps) -> Html {
    let mnemonic = LocalStorage::get(KEY).unwrap_or_else(|_| {
        LocalStorage::set(KEY, "").ok();
        "".to_string()
    });
    let is_invalid_mnemonic = use_state(|| mnemonic.is_empty());
    let words = if mnemonic.is_empty() {
        use_state(|| {
            let mut vec = Vec::<String>::new();
            for _ in 0..12 {
                vec.push("".to_string());
            }
            vec
        })
    } else {
        use_state(|| {
            mnemonic
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
    };

    let onchanged = {
        Callback::from(&move |message: String| {
            log::info!("onchanged {}", message);
        })
    };
    let onclick_generate_keys_button = {
        let is_invalid_mnemonic = is_invalid_mnemonic.clone();
        let words = words.clone();
        Callback::from(move |_| {
            let new_words: Vec<String> = Mnemonic::generate_in(Language::English, 12)
                .unwrap()
                .to_string()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            words.set(new_words);
            is_invalid_mnemonic.set(false); // Generated mnemonic must be valid.
        })
    };
    let onclick_revert_form_button = {
        let is_invalid_mnemonic = is_invalid_mnemonic.clone();
        let words = words.clone();
        Callback::from(move |_| {
            if !mnemonic.is_empty() {
                words.set(
                    mnemonic
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                );
                is_invalid_mnemonic.set(false);
            }
        })
    };
    let onclick_clear_form_button = {
        let is_invalid_mnemonic = is_invalid_mnemonic.clone();
        let words = words.clone();
        Callback::from(move |_| {
            let mut new_words = Vec::<String>::new();
            for _ in 0..12 {
                new_words.push("".to_string());
            }
            words.set(new_words);
            is_invalid_mnemonic.set(true); // Empty mnemonic will be invalid.
        })
    };
    let onclick_open_wallet_button = {
        let is_invalid_mnemonic = is_invalid_mnemonic.clone();
        let words = words.clone();
        Callback::from(move |_| {
            let str = words.clone().join(" ");
            match Mnemonic::parse(&str) {
                Ok(mnemonic) => {
                    let xkey: ExtendedKey = mnemonic
                        .clone()
                        .into_extended_key()
                        .expect("a valid key should have been provided");
                    let pubkey = xkey
                        .into_xpub(Network::Testnet, &Secp256k1::new())
                        .to_string();
                    log::info!("{:#}", mnemonic);
                    let wallet = WalletParams {
                        mnemonic: str.to_string(),
                        pubkey,
                    };
                    let client = reqwest::Client::new();
                    spawn_local(async move {
                        let res = client
                            .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet")
                            //.put("http://localhost:8080/wallet")
                            .json(&wallet)
                            .send()
                            .await;
                        log::info!("{:#?}", res);
                    });
                }
                Err(_) => is_invalid_mnemonic.set(true),
            };
            LocalStorage::set(KEY, str).ok();
        })
    };

    html! {
        <>
            <h1>{"Fill your mnemonic word in."}</h1>
            <MatList>
                <MnemonicWordList words={(*words).clone()} {onchanged}/>
                <GenerateKeysButton onclick={onclick_generate_keys_button}/>
                <OpenWalletButton onclick={onclick_open_wallet_button} disabled={(*is_invalid_mnemonic).clone()}/>
                <ClearFormButton onclick={onclick_clear_form_button}/>
                <RevertFormButton onclick={onclick_revert_form_button}/>
            </MatList>
        </>
    }
}

#[derive(Serialize, Deserialize)]
pub struct WalletParams {
    mnemonic: String,
    pubkey: String,
}

pub struct Page {}

impl Component for Page {
    type Properties = ();
    type Message = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <MnemonicPageInner/>
        }
    }
}

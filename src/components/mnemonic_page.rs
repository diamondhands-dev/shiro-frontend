use bdk::bitcoin::{secp256k1::Secp256k1, Network};
use bdk::keys::bip39::{Language, Mnemonic};
use bdk::keys::{DerivableKey, ExtendedKey};
use gloo::storage::{LocalStorage, Storage};
use material_yew::{MatButton, MatCircularProgress, MatIcon, MatList, MatListItem, MatTextField};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, prelude::*, use_state, Html, Properties};

const KEY: &str = "shiro.mnemonic";
const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Deserialize, Serialize)]
pub struct WalletResult {
    pub mnemonic: String,
    pub pubkey: String,
}

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
            <div class="small1">
                <MatTextField size={100} outlined=true label={props.label.clone()} value={props.value.clone()} oninput={props.oninput.clone()} />
            </div>
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
                    onchanged.emit(message);
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
        //let navigator = use_history().unwrap();
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            e.default_prevented();
            onclick.emit(e);
            //navigator.push(crate::Route::Balance);
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
    let open = use_state(|| false);
    let created = use_state(|| false);
    let online = use_state(|| false);
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
        let open = open.clone();
        let created = created.clone();
        let online = online.clone();
        Callback::from(move |_| {
            let baseurl = web_sys::window().unwrap().origin();
            let str = words.clone().join(" ");
            let is_invalid_mnemonic = is_invalid_mnemonic.clone();
            let open = open.clone();
            let created = created.clone();
            let online = online.clone();
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
                    {
                        let wallet = WalletParams {
                            mnemonic: str.to_string(),
                            pubkey,
                        };
                        let go_online = GoOnlineParams {
                            skip_consistency_check: true,
                            electrum_url: "shiro.westus2.cloudapp.azure.com:50001".to_string(),
                        };
                        let client = reqwest::Client::new();
                        spawn_local(async move {
                            open.set(true);
                            let res = client
                                .put(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet")
                                .json(&wallet)
                                .send()
                                .await;
                            log::info!("{:#?}", res);
                            match res {
                                Ok(res) => match res.json::<WalletResult>().await {
                                    Ok(_json) => {
                                        created.set(true);
                                    }
                                    Err(e) => {
                                        log::error!("{:?}", e);
                                        created.set(false);
                                    }
                                },
                                Err(e) => {
                                    log::error!("{:#?}", e);
                                    created.set(false);
                                }
                            }

                            let res = client
                                .put(
                                    API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned()
                                        + "/wallet/go_online",
                                )
                                .json(&go_online)
                                .send()
                                .await;
                            log::info!("{:#?}", res);
                            match res {
                                Ok(res) => match res.text().await {
                                    Ok(json) => {
                                        log::info!("go_online {:#?}", json);
                                        online.set(true);
                                    }
                                    Err(e) => {
                                        log::error!("{:?}", e);
                                        online.set(false);
                                    }
                                },
                                Err(e) => {
                                    log::error!("{:#?}", e);
                                    online.set(false);
                                }
                            }
                            open.set(false);
                        });
                    }
                }
                Err(_) => is_invalid_mnemonic.set(true),
            };
            LocalStorage::set(KEY, str).ok();
        })
    };

    html! {
        <>
            <p>{"Fill your mnemonic word in."}</p>

            <MatList>
                    <MnemonicWordList words={(*words).clone()} {onchanged}/>
            </MatList>

            <div class="box">
            if *open {
                <MatCircularProgress indeterminate=true />
            } else {
                <OpenWalletButton onclick={onclick_open_wallet_button} disabled={*is_invalid_mnemonic}/>
            }
            </div>

            <div class="box">
                {"Wallet created"}
                if *created {
                    <MatIcon>{"check_circle"}</MatIcon>
                } else {
                    <MatIcon>{"block"}</MatIcon>
                }
            </div>

            <div class="box">
                {"Wallet online"}
                if *online {
                    <MatIcon>{"check_circle"}</MatIcon>
                } else {
                    <MatIcon>{"block"}</MatIcon>
                }
            </div>

            <div class="box">
                <GenerateKeysButton onclick={onclick_generate_keys_button}/>
                <ClearFormButton onclick={onclick_clear_form_button}/>
                <RevertFormButton onclick={onclick_revert_form_button}/>
            </div>

        </>
    }
}

#[derive(Serialize, Deserialize)]
pub struct WalletParams {
    mnemonic: String,
    pubkey: String,
}

#[derive(Deserialize, Serialize)]
pub struct GoOnlineParams {
    skip_consistency_check: bool,
    electrum_url: String,
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

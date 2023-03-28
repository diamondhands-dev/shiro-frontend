use material_yew::{MatButton, MatCircularProgress};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const API_ROOT: Option<&'static str> = option_env!("API_ROOT");

#[derive(Serialize, Deserialize)]
pub struct RefreshParams {
    asset_id: Option<String>,
    filter: Vec<RefreshFilter>,
}

#[derive(Deserialize, Serialize)]
struct RefreshFilter {
    status: String,
    incoming: bool,
}

#[function_component(RefreshButton)]
pub fn refresh_button() -> Html {
    let refresh = use_state(|| false);
    let onclick = {
        let refresh = refresh.clone();
        Callback::from(move |_: MouseEvent| {
            let baseurl = web_sys::window().unwrap().origin();
            let refresh = refresh.clone();
            spawn_local(async move {
                refresh.set(true);
                match reqwest::Client::new()
                    .post(API_ROOT.unwrap_or(&baseurl.to_owned()).to_owned() + "/wallet/refresh")
                    .json(&RefreshParams {
                        asset_id: None,
                        filter: Vec::<RefreshFilter>::new(),
                    })
                    .send()
                    .await
                {
                    Ok(result) => {
                        log::info!("{:?}", result);
                        refresh.set(false);
                    }
                    Err(e) => {
                        log::error!("{:#?}", e);
                        refresh.set(false);
                    }
                };
            });
        })
    };

    html! {
        if *refresh {
            <MatCircularProgress indeterminate=true />
        } else {
            <div {onclick}>
                <MatButton label={"Refresh"} />
            </div>
        }
    }
}

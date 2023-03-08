use material_yew::MatButton;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
    let onclick = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
            match reqwest::Client::new()
                .put("http://shiro.westus2.cloudapp.azure.com:4320/wallet/refresh")
                .json(&RefreshParams {
                    asset_id: None,
                    filter: Vec::<RefreshFilter>::new(),
                })
                .send()
                .await
            {
                Ok(result) => log::info!("{:?}", result),
                Err(e) => log::error!("{:#?}", e),
            };
        });
    });

    html! {
        <div {onclick}>
            <MatButton label={"Refresh"} />
        </div>
    }
}

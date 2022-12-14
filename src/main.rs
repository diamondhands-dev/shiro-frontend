use yew::prelude::*;
use yew_router::prelude::*;

use self::components::balance_tab::BalanceTab;
use self::components::utxos_page::Page as UtxosPage;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    BalanceTabRoute,
    #[at("/utxos")]
    UtxosPageRoute,
}

#[function_component(BalanceTabRoute)]
fn balance_tab() -> Html {
    html! {
        <section>
            <BalanceTab/>
        </section>
    }
}

#[function_component(UtxosPageRoute)]
fn utxo_page() -> Html {
    html! {
        <section>
            <UtxosPage/>
        </section>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::BalanceTabRoute => html!( <BalanceTabRoute/> ),
        Route::UtxosPageRoute => html!( <UtxosPageRoute/> ),
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}

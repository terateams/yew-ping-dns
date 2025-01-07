use yew::{function_component, html, Html, Renderer};
mod components;
use components::{dns::DnsQuery, ping::Ping};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Ping and DNS Simulation" }</h1>
            <Ping />
            <DnsQuery />
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
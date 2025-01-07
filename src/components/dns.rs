use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request; // 修改为 gloo_net

#[function_component(DnsQuery)]
pub fn dns_query() -> Html {
    let domain = use_state(|| String::from(""));
    let result = use_state(|| String::from(""));

    let on_input = {
        let domain = domain.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                domain.set(input.value());
            }
        })
    };

    let on_click = {
        let domain = domain.clone();
        let result = result.clone();
        Callback::from(move |_| {
            let domain = domain.clone();
            let result = result.clone();
            spawn_local(async move {
                let url = format!("https://dns.google/resolve?name={}&type=A", *domain);
                match Request::get(&url).send().await {
                    Ok(response) => {
                        if let Ok(body) = response.text().await {
                            result.set(format!("DNS Query for {} succeeded! Response: {}", *domain, body));
                        } else {
                            result.set("Failed to parse DNS response.".into());
                        }
                    }
                    Err(_) => {
                        result.set("DNS Query failed due to network error.".into());
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <h2 class="title">{ "DNS Query Simulator" }</h2>
            <input class="input" type="text" placeholder="Enter domain" oninput={on_input} />
            <button class="button" onclick={on_click}>{ "Query" }</button>
            <p class="result">{ (*result).clone() }</p>
        </div>
    }
}
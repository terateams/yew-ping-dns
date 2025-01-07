use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;

#[function_component(Ping)]
pub fn ping() -> Html {
    let url = use_state(|| String::from(""));
    let result = use_state(|| String::from(""));

    let on_input = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                url.set(input.value());
            }
        })
    };

    let on_click = {
        let url = url.clone();
        let result = result.clone();
        Callback::from(move |_| {
            let url = url.clone();
            let result = result.clone();
            spawn_local(async move {
                let url_trimmed = url.trim();

                if url_trimmed.is_empty() {
                    result.set("Error: URL cannot be empty!".to_string());
                    return;
                }

                // 自动补全 http 或 https
                let formatted_url = if url_trimmed.starts_with("http://") || url_trimmed.starts_with("https://") {
                    url_trimmed.to_string()
                } else {
                    format!("https://{}", url_trimmed) // 默认使用 https
                };

                result.set(format!("Pinging {}...", formatted_url));

                // 发送 HTTP GET 请求
                match Request::get(&formatted_url).send().await {
                    Ok(_) => {
                        result.set(format!("Ping to {} succeeded! Website is reachable.", formatted_url));
                    }
                    Err(_) => {
                        result.set(format!("Ping to {} failed! Website is unreachable.", formatted_url));
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Ping URL Simulator" }</h2>
            <input type="text" placeholder="Enter URL (e.g., example.com)" oninput={on_input} />
            <button onclick={on_click}>{ "Ping" }</button>
            <p>{ (*result).clone() }</p>
        </div>
    }
}
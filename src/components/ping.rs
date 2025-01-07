use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::{Request, Method};
use web_sys::js_sys::Date;


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
                let start = Date::now(); // 记录开始时间
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

                // 发送 HTTP HEAD 请求，仅判断是否可以到达
                match Request::get(&formatted_url)
                    .method(Method::HEAD)
                    .mode(web_sys::RequestMode::NoCors)
                    .send()
                    .await {
                    Ok(response) => {
                        let end = Date::now(); // 记录结束时间
                        let duration = end - start;
                        let status = response.status();
                        result.set(format!("Ping to {} succeeded! Status: {}. 响应时间: {:.2} ms", formatted_url, status, duration));
                    }
                    Err(_) => {
                        let end = Date::now();
                        let duration = end - start;
                        result.set(format!("Ping to {} failed! Website is unreachable. 响应时间: {:.2} ms", formatted_url, duration));
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
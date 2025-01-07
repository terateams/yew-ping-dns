use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture; // 这里是正确的导入方式

#[function_component(Ping)]
pub fn ping() -> Html {
    let host = use_state(|| String::from(""));
    let result = use_state(|| String::from(""));

    let on_input = {
        let host = host.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                host.set(input.value());
            }
        })
    };

    let on_click = {
        let host = host.clone();
        let result = result.clone();
        Callback::from(move |_| {
            let host = host.clone();
            let result = result.clone();
            spawn_local(async move {
                TimeoutFuture::new(1000).await; // 使用 gloo_timers 的 TimeoutFuture 模拟延迟
                result.set(format!("Ping to {} succeeded! Response time: 23ms", *host));
            });
        })
    };

    html! {
        <div>
            <h2>{ "Ping Simulator" }</h2>
            <input type="text" placeholder="Enter host" oninput={on_input} />
            <button onclick={on_click}>{ "Ping" }</button>
            <p>{ (*result).clone() }</p>
        </div>
    }
}
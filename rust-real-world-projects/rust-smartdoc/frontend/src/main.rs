use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;


#[derive(Serialize)]
struct DocumentationRequest {
    function_name: String,
}

#[derive(Deserialize, Debug)]
struct DocumentationResponse {
    description: String,
    example: String,
}

#[function_component(App)]
fn app() -> Html {
    let function_name = use_state(|| String::new());
    let documentation = use_state(|| None as Option<DocumentationResponse>);

    let on_input = {
        let function_name = function_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                function_name.set(input.value());
            }
        })
    };

    let fetch_documentation = {
        let function_name = function_name.clone();
        let documentation = documentation.clone();

        Callback::from(move |_| {
            let function_name = (*function_name).clone();
            let documentation = documentation.clone();

            spawn_local(async move {
                let request = DocumentationRequest { function_name };
                let response: DocumentationResponse = Request::post("http://127.0.0.1:8080/documentation")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                documentation.set(Some(response));
            });
        })
    };

    html! {
        <div>
            <h1>{ "Rust SmartDoc" }</h1>
            <input
                type="text"
                placeholder="Enter function name"
                value={(*function_name).clone()}
                oninput={on_input}
            />
            <button onclick={fetch_documentation}>{ "Fetch Documentation" }</button>

            {
                if let Some(doc) = (*documentation).clone() {
                    html! {
                        <div>
                            <h2>{ "Description" }</h2>
                            <p>{ doc.description }</p>
                            <h2>{ "Example" }</h2>
                            <pre>{ doc.example }</pre>
                        </div>
                    }
                } else {
                    html! { <p>{ "No documentation fetched yet." }</p> }
                }
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}

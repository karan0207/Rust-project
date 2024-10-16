use yew::prelude::*;

/// Model struct holding the state of the app
struct App;

/// The message type for the app
enum Msg {
    Clicked,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                // Perform any action when the button is clicked
                web_sys::console::log_1(&"Button clicked!".into());
                true // Re-render component after update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Hello, Yew Frontend!" }</h1>
                <button onclick={ctx.link().callback(|_| Msg::Clicked)}>{ "Click me!" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node, window, Window};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div id="ext-tab-bar"></div>
        <div id="ext-main"></div>
        <div id="ext-dev" hidden=true></div>
        <script type="module">
            {"import {joinRoom} from 'https://cdn.skypack.dev/trystero'; extension.getBackgroundPage().joinRoom = joinRoom;"}
        </script>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
}

fn main() {
    let app = yew::start_app::<App>();
}
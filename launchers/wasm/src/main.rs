use bevy::prelude::*;
use stylist::yew::styled_component;
use stylist::{css, global_style};
use yew::prelude::*;

use my_game::theme::LAUNCHER_TITLE;

fn set_window_title(title: &str) {
    web_sys::window()
        .map(|w| w.document())
        .flatten()
        .expect("Unable to get DOM")
        .set_title(title);
}

fn set_global_css() {
    global_style! {
        r#"
        html {
            min-height: 100%;
            position: relative;
        }
        body {
            height: 100%;
            padding: 0;
            margin: 0;
        }
        "#
    }
    .expect("Unable to mount global style");
}

#[styled_component(Root)]
fn view() -> Html {
    set_window_title(LAUNCHER_TITLE);
    set_global_css();

    let loading = use_state(|| false);
    let onclick = {
        let loading = loading.clone();
        Callback::from(move |_| {
            loading.set(true);
            start_bevy();
        })
    };

    let css = css!(
        r#"
        position: absolute;
        overflow: hidden;
        width: 100%;
        height: 100%;
        "#
    );

    html! {
        <div class={ css }>
        if *loading {
            <canvas id="bevy"></canvas>
        } else {
            <button {onclick}>{"Start Game"}</button>
        }
        </div>
    }
}

fn start_bevy() {
    // Start the Bevy App
    let mut app = my_game::app();
    info!("Starting launcher: WASM");
    app.run();
}

fn main() {
    // Mount the DOM
    yew::start_app::<Root>();
}

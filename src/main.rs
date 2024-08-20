/*
Use:
for testing:
    -dx serve --platform web --port 8060
for production:
    -dx serve --platform web --port 8060 --target production
    -execute the build
*/

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use crate::components::{headerfooter::HeaderFooter, home::Home, pagenotfound::PageNotFound};

mod components {
    pub mod headerfooter;
    pub mod home;
    pub mod pagenotfound;
}
static WINDOW_DIMS: GlobalSignal<(f64, f64)> = Signal::global(|| (0.0, 0.0));

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:route")]
    PageNotFound {
        route: String,
    },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    // Start screen size watcher
    use_future(move || async move {
        let mut eval = eval(
            r"
            function resize() {
                dioxus.send([window.innerWidth, window.innerHeight]);
            }
            window.addEventListener('resize', resize);
            dioxus.send([window.innerWidth, window.innerHeight]);
        ",
        );
        loop {
            let response = eval.recv().await.unwrap();
            let dims: (f64, f64) = serde_json::from_value(response).unwrap();
            *WINDOW_DIMS.write() = (dims.0, dims.1);
        }
    });

    rsx!(
        div {
            class: "bg-gradient-to-b from-green-400 to-blue-500",

            div {class:"hidden",
                width: "100%",
                height: "50%",
                background_color: "red",
                "This element is {WINDOW_DIMS():?}"
            }
            div {class:"container mx-auto min-h-screen min-w-screen",

                div { class:"py-24",
                    Router::<Route> {},
                }

            }
    }
    )
}

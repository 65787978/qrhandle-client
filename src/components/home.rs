use crate::components::card::Card;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {{Card()}}
}

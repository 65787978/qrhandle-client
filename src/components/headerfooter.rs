use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn HeaderFooter() -> Element {
    let mut dropdown_menu_toggle = use_signal(|| false);
    let mut dropdown_menu_style = use_signal(|| {
        "visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"
    });

    /* Header */
    rsx! {

            nav {class:"fixed top-4 left-0 right-0 z-10 bg-opacity-25 bg-black backdrop-filter backdrop-blur-md rounded-full shadow-lg py-2 mx-4", id:"navbar-default",

                div {class:"grid grid-cols-3 sm:grid-cols-1 justify-items-center items-center ps-2 mx-2",

                    /* Small screen navbar text */
                    div {class:"sm:hidden col-start-1 col-span-1 justify-self-start text-slate-200 text-bold text-xl", Link {to: Route::Home {}, "QR Handle"}}

                    /* Small screen navbar-button */
                    button { onclick: move |_| {
                            if dropdown_menu_toggle() {
                                dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(false);
                            }
                            else {
                                dropdown_menu_style.set("visibility: visible; opacity: 1; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(true);
                            }
                        },
                        r#type:"button", class:" sm:hidden col-start-3 col-span-1 justify-self-end items-center p-2 w-10 h-10 text-sm text-slate-200 rounded-full hover:bg-slate-100/50 focus:outline-none focus:ring-2 focus:ring-slate-200",
                            svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                                path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                            }
                    }

                    /* Small screen navbar-menu */
                    div {
                        div {
                            class: "sm:hidden absolute right-0 z-50 m-4 w-56 origin-top-right bg-opacity-25 bg-black backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2 justify-end items-center text-center content-center ",
                            style: "{dropdown_menu_style}",
                            id: "dropdown_menu",
                            div {class:"grid grid-rows-auto justify-center items-center py-2",
                                div { class:"py-2", Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);}, to: Route::Home {}, class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold", "Home"}}

                                div { class:"py-2", Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);}, to: Route::Home {}, class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold", "Create"}}

                                div { class:"py-2", Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);}, to:"", class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold", "Support"}}

                            }
                        }
                    }


                    /* Large screen navbar  */
                    div {class:"hidden sm:block sm:grid sm:grid-cols-3 justify-items-center items-center text-center content-center sm:h-fit sm:w-full",
                        div { Link {to: Route::Home {}, class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold m-2 ", "Home"}
                        }

                        div { Link {to: Route::Home {}, class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold m-2 ", "Create"}
                        }

                        div { Link {to:"", class:"text-slate-200 rounded-lg hover:text-slate-100 hover:font-bold m-2 ", "Support"}
                        }
                    }
                }
        }
        Outlet::<Route> {}

        /* Footer */
        div {class:"min-h-[2rem] inset-x-0 bottom-0 bg-opacity-25 bg-black backdrop-filter backdrop-blur-md fixed",
            div {class:"flex flex-row justify-center",
                    a {class:"icon m-2", href:"https://github.com/th3-cr34t0r/qrhandle-client",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-github", "viewBox":"0 0 16 16",
                        path {d:"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8"}
                    }
                }

                a {class:"icon m-2", href:"",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-discord", "viewBox":"0 0 16 16",
                        path {d:"M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"}
                    }
                }

                a {class:"icon m-2", href:"",
                    svg {
                        xmlns:"http://www.w3.org/2000/svg", width:"25", height:"25", fill:"white", class:"bi bi-telegram", "viewBox":"0 0 16 16",
                        path {d:"M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M8.287 5.906q-1.168.486-4.666 2.01-.567.225-.595.442c-.03.243.275.339.69.47l.175.055c.408.133.958.288 1.243.294q.39.01.868-.32 3.269-2.206 3.374-2.23c.05-.012.12-.026.166.016s.042.12.037.141c-.03.129-1.227 1.241-1.846 1.817-.193.18-.33.307-.358.336a8 8 0 0 1-.188.186c-.38.366-.664.64.015 1.088.327.216.589.393.85.571.284.194.568.387.936.629q.14.092.27.187c.331.236.63.448.997.414.214-.02.435-.22.547-.82.265-1.417.786-4.486.906-5.751a1.4 1.4 0 0 0-.013-.315.34.34 0 0 0-.114-.217.53.53 0 0 0-.31-.093c-.3.005-.763.166-2.984 1.09"}
                    }
                }
            }
        }
    }
}

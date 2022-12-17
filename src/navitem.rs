use yew::{
    classes, function_component, html, use_state, AttrValue, Callback, Html, NodeRef, Properties,
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(false)]
    pub active: bool,
    pub child: yew::virtual_dom::VNode,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(false)]
    pub do_not_collapse: bool,
    #[prop_or(AttrValue::from("javascript:;"))]
    pub href: AttrValue,
    pub onclick: Option<Callback<yew::MouseEvent>>,
    pub submenu: Option<Vec<SubMenuItemProps>>,
    pub submenu_navbar_collapser: Option<Callback<yew::MouseEvent>>,
}

#[function_component]
pub fn NavItem(props: &Props) -> Html {
    let node_ref = NodeRef::default();
    let submenu_expanded = use_state(|| false);
    // here, we need to handle activating the submenu (if applicable) while also handling the
    // passed onclick
    let onclick = {
        let onclick = props.onclick.clone();
        let submenu = props.submenu.clone();
        let submenu_expanded = submenu_expanded.clone();
        Callback::from(move |e: yew::MouseEvent| {
            match onclick.clone() {
                Some(callback) => {
                    callback.emit(e);
                }
                None => {}
            }
            {
                match &submenu {
                    Some(_) => {
                        submenu_expanded.set(!*submenu_expanded);
                    }
                    None => {}
                }
            }
        })
    };
    let onfocusout = {
        // clicking outside of submenu should hide the submenu,
        // but submenu clicks should not hide it
        // this same callback is also called if a submenu item with do_not_collpase=true
        // loses focus
        let submenu_expanded = submenu_expanded.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |_| {
            let submenu_expanded = submenu_expanded.clone();
            let node_ref = node_ref.clone();
            // everything needs to happen behind a timer
            // because the newly focused element is not known until the next tick
            let timer = gloo_timers::callback::Timeout::new(0, move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(active_element) = document.active_element() {
                            // determine if the active element is a child of the node_ref
                            let self_el: &web_sys::Element =
                                &node_ref.cast().expect("could not get node_ref element");
                            let mut is_child = false;
                            let mut parent = active_element.parent_element();
                            while parent != None {
                                if parent.as_ref() == Some(self_el) {
                                    is_child = true;
                                    break;
                                }
                                parent = parent.expect("hard-coded fail").parent_element();
                            }
                            // submenu should not collapse if the newly focused element
                            // is a child of the node_ref
                            if !is_child {
                                submenu_expanded.set(false);
                            }
                        }
                    }
                };
            });
            timer.forget(); // this executes the timer, and is poorly named, IMO
        })
    };
    html! {
        <li
            ref={node_ref}
            class={classes!(
                "nav-item",
                match &props.submenu {
                    Some(_) => Some("dropdown"),
                    None => None
                }
                )}
            >
            <a
                class={classes!(
                    "nav-link",
                    match props.active {
                        true => Some("active"),
                        _ => None
                    },
                    match props.disabled {
                        true => Some("disabled"),
                        _ => None
                    },
                    match &props.submenu {
                        Some(_) => Some("dropdown-toggle"),
                        _ => None
                    }
                )}
                href={&props.href}
                onclick={onclick}
                onfocusout={onfocusout.clone()}
            >{props.child.clone()}</a>
            {match &props.submenu {
                Some(submenu) => {
                    Some(html! {
                        <div class={classes!(
                            "dropdown-menu",
                            match *submenu_expanded {
                                true => Some("d-block"),
                                _ => None
                            }
                            )}>
                            {submenu.into_iter().map(|submenu_item| {
                                // here, we need to handle deactivating the submenu while also
                                // handling the passed onclick
                                let onclick = {
                                    let onclick = submenu_item.onclick.clone();
                                    let submenu_navbar_collapser = props.submenu_navbar_collapser.clone();
                                    let submenu_expanded = submenu_expanded.clone();
                                    let do_not_collapse = submenu_item.do_not_collapse;
                                    Callback::from(move |e: yew::MouseEvent| {
                                        match &onclick {
                                            Some(onclick) => {
                                                onclick.emit(e.clone());
                                            },
                                            None => {}
                                        }
                                        if !do_not_collapse {
                                            if *submenu_expanded {}
                                            submenu_expanded.set(false);
                                            match &submenu_navbar_collapser {
                                                Some(submenu_navbar_collapser) => {
                                                    submenu_navbar_collapser.emit(e);
                                                },
                                                None => {}
                                            }
                                        }
                                    })
                                };
                                html! {
                                    <SubMenuItem
                                        child={submenu_item.child.clone()}
                                        onclick={onclick}
                                        onfocusout={onfocusout.clone()}
                                    />
                                }
                            }).collect::<Html>()}
                        </div>
                    })
                },
                None => None
            }}
        </li>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SubMenuItemProps {
    #[prop_or(false)]
    pub active: bool,
    pub child: yew::virtual_dom::VNode,
    #[prop_or(false)]
    pub do_not_collapse: bool, // used by the navbar
    #[prop_or(AttrValue::from("javascript:;"))]
    pub href: AttrValue,
    pub onclick: Option<Callback<yew::MouseEvent>>,
    // handles submenu collapse when do_not_collapse is true and item loses focus
    pub onfocusout: Option<Callback<yew::FocusEvent>>,
}

#[function_component]
pub fn SubMenuItem(props: &SubMenuItemProps) -> Html {
    html! {
        <a
            class={classes!(
                "dropdown-item"
            )}
            href={&props.href}
            onclick={props.onclick.clone()}
            onfocusout={props.onfocusout.clone()}
        >{props.child.clone()}</a>
    }
}

use crate::navitem::{NavItem, Props as NavItemProps};
use yew::{classes, function_component, html, use_state, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub brand: Option<BrandProps>,
    #[prop_or(false)]
    pub collapsible: bool,
    #[prop_or(None)]
    pub fixed_to: Option<FixedTo>,
    pub fluid: Option<bool>,
    #[prop_or(vec![])]
    pub navitems: Vec<NavItemProps>,
    #[prop_or(TogglerPosition::Right)]
    pub toggler_position: TogglerPosition,
}

#[function_component]
pub fn NavBar(props: &Props) -> Html {
    let expand_state = use_state(|| false);
    let toggler_handler = {
        let expand_state = expand_state.clone();
        Callback::from(move |_e: yew::MouseEvent| {
            expand_state.set(!*expand_state);
        })
    };
    let container_class = match props.fluid {
        Some(true) => "container-fluid",
        _ => "container",
    };
    let collapse_class = match props.collapsible {
        true => classes!(
            "collapse",
            "navbar-collapse",
            match *expand_state {
                true => Some("d-block"),
                _ => None,
            }
        ),
        _ => classes!(),
    };
    let navitems = props.navitems.clone();
    html! {
    <nav class={classes!(
        "navbar",
        match &props.fixed_to {
            Some(fixed_to) => match fixed_to {
                FixedTo::Top => Some("fixed-top"),
                FixedTo::Bottom => Some("fixed-bottom"),
            }
            None => None
        },
        match props.collapsible {
            true => "navbar-expand-md",
            _ => "navbar-expand"
        },
        "navbar-dark",
        "bg-dark",
    )}>
        <div class={container_class}>
            {match props.toggler_position {
                TogglerPosition::Left => {
                    Some(html! { <Toggler onclick={toggler_handler.clone()} /> })
                },
                _ => None,
            }}
            {match &props.brand {
                Some(brand) => Some(html! {
                    <Brand
                        link={brand.link}
                        href={brand.href.clone()}
                        child={brand.child.clone()}
                        onclick={brand.onclick.clone()}
                    />
                }),
                _ => None
            }}
            {match props.toggler_position {
                TogglerPosition::Right => {
                    Some(html! { <Toggler onclick={toggler_handler.clone()} /> })
                },
                _ => None,
            }}
            <div class={collapse_class}>
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                { navitems.into_iter().map(|navitem| {
                    // here, we need a way to handle the navitem's onclick
                    // before collapsing the navbar on mobile
                    let navitem_onclick = navitem.onclick.clone();
                    let expand_state = expand_state.clone();
                    let submenu = navitem.submenu.clone();
                    let onclick = {
                        let toggler_handler = toggler_handler.clone();
                        let expand_state = expand_state.clone();
                        Callback::from(move |e: yew::MouseEvent| {
                            match &navitem_onclick {
                                Some(onclick) => {
                                    onclick.emit(e.clone());
                                },
                                _ => {}
                            }
                            if !navitem.do_not_collapse {
                                match submenu {
                                    Some(_) => {},
                                    _ => {
                                        match *expand_state {
                                            true => {
                                                toggler_handler.emit(e);
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        })
                    };
                    let submenu_navbar_collapser = {
                        let toggler_handler = toggler_handler.clone();
                        let expand_state = expand_state.clone();
                        Callback::from(move |e: yew::MouseEvent| {
                            if *expand_state {
                                toggler_handler.emit(e);
                            }
                        })
                    };
                    html! {
                        <NavItem
                            active={navitem.active}
                            child={navitem.child}
                            disabled={navitem.disabled}
                            href={navitem.href}
                            onclick={onclick}
                            submenu={navitem.submenu}
                            submenu_navbar_collapser={submenu_navbar_collapser}
                        />
                    }
                }).collect::<Html>() }
                </ul>
            </div>
        </div>
    </nav>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrandProps {
    #[prop_or(false)]
    pub link: bool,
    #[prop_or(AttrValue::from("javascript:;"))]
    pub href: AttrValue,
    pub child: yew::virtual_dom::VNode,
    pub onclick: Option<Callback<yew::MouseEvent>>,
}

#[function_component]
pub fn Brand(props: &BrandProps) -> Html {
    html! {
        <div>
            <a
                class={classes!("navbar-brand", match props.link {
                    true => None,
                    _ => Some("mb-0 h1"),
                })}
                href={&props.href}
                onclick={props.onclick.clone()}
            >{props.child.clone()}</a>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TogglerProps {
    onclick: Callback<yew::MouseEvent>,
}

#[function_component]
fn Toggler(props: &TogglerProps) -> Html {
    html! {
        <button class="navbar-toggler" type="button" onclick={props.onclick.clone()}
        ><span class="navbar-toggler-icon"></span></button>
    }
}

#[derive(PartialEq, Clone)]
pub enum FixedTo {
    Top,
    Bottom,
}

#[derive(PartialEq, Clone)]
pub enum TogglerPosition {
    Left,
    Right,
}

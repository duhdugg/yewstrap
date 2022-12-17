use yew::prelude::*;
use yewstrap::appdiv::AppDiv;
use yewstrap::card::Card;
use yewstrap::navbar::{
    BrandProps as NavBarBrandProps, FixedTo as NavBarFixedTo, Props as NavBarProps,
};
use yewstrap::navitem::{Props as NavItemProps, SubMenuItemProps};
use yewstrap::spinner::{Spinner, FlexDirection, SpinnerType};

#[function_component]
fn App() -> Html {
    let greeting = use_state(|| AttrValue::from("Hello, world!"));
    let active_navitem = use_state(|| 0);
    let onclick = || {
        let greeting = greeting.clone();
        Callback::from(move |_| greeting.set(AttrValue::from(format!("{}!", *greeting))))
    };
    html! {
        <AppDiv
            navbar_props={yew::props! { NavBarProps{
                collapsible: true,
                brand: yew::props! { NavBarBrandProps {
                    href: AttrValue::from(""),
                    child: html! { <span>{"Yewstrap"}</span> },
                    onclick: {
                        let active_navitem = active_navitem.clone();
                        Callback::from(move |e: yew::MouseEvent| {
                            e.prevent_default();
                            active_navitem.set(0);
                            log::debug!("Yewstrap");
                        })
                    },
                }},
                fixed_to: NavBarFixedTo::Top,
                navitems: {
                    vec![1,2,3,4].into_iter().map(|i| {
                        let active_navitem = active_navitem.clone();
                        let s = match i {
                            1 => "ONE",
                            2 => "TWO",
                            3 => "III",
                            _ => "FOO",
                        };
                        yew::props! { NavItemProps {
                            active: *active_navitem == i,
                            href: AttrValue::from(""),
                            child: html! { <span>{s}</span> },
                            onclick: Callback::from(move |e: yew::MouseEvent| {
                                e.prevent_default();
                                active_navitem.set(i);
                                log::debug!("{}", s);
                            }),
                            submenu: match i {
                                4 => {
                                    Some(vec![
                                        yew::props! { SubMenuItemProps {
                                            child: html! { <span>{i}{"-Alpha"}</span> },
                                            onclick: Callback::from(|e: yew::MouseEvent| {
                                                e.prevent_default();
                                                log::debug!("Alpha");
                                            })
                                        }},
                                        yew::props! { SubMenuItemProps {
                                            child: html! { <span>{i}{"-Bravo"}</span> },
                                            do_not_collapse: true,
                                            onclick: Callback::from(|e: yew::MouseEvent| {
                                                e.prevent_default();
                                                log::debug!("Bravo");
                                            })
                                        }}
                                    ])
                                },
                                _ => None
                            }
                        }}
                    }).collect::<Vec<NavItemProps>>()
                },
            }}}
            header={html! {
                <h1>{&*greeting}</h1>
            }}
            footer={html! {
                <p class="pt-3">{ "Copyright Doug Elkin © 2022. All rights reserved." }</p>
            }}
        >
            <div class="btn-group">
                <button onclick={onclick()} class="btn btn-primary">{ "Click" }</button>
            </div>
            <div class="row mt-3">
                <Card>
                    <h2>{ "Spinners" }</h2>
                    <Spinner
                        flex_direction={FlexDirection::Row}
                        size={1.333}
                        spinner_type={SpinnerType::Border}
                    >{"plz wait..."}</Spinner>
                    <Spinner
                        flex_direction={FlexDirection::Row}
                        size={1.333}
                        spinner_type={SpinnerType::Grow}
                    >{"Loading..."}</Spinner>
                    <div>
                        <Spinner
                            flex_direction={FlexDirection::Column}
                            size={3.0}
                        >{"Hold Up!"}</Spinner>
                    </div>
                </Card>
            </div>
        </AppDiv>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

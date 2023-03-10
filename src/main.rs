use yew::prelude::*;
use yewstrap::alert::{Alert, Theme as AlertTheme};
use yewstrap::appdiv::AppDiv;
use yewstrap::card::{Card, Theme as CardTheme};
use yewstrap::modal::{Modal, Theme as ModalTheme};
use yewstrap::navbar::{
    BrandProps as NavBarBrandProps, FixedTo as NavBarFixedTo, Props as NavBarProps,
};
use yewstrap::navitem::{Props as NavItemProps, SubMenuItemProps};
use yewstrap::spinner::{FlexDirection, Spinner, SpinnerType};
use yewstrap::{BreakpointWidths, ColumnWidth};

#[function_component]
fn App() -> Html {
    let greeting = use_state(|| AttrValue::from("Hello, world!"));
    let active_navitem = use_state(|| 0);
    let foo_bravo_toggler = use_state(|| false);
    let show_modal = use_state(|| false);
    let greeting_onclick = || {
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
                                            child: html! { <span>
                                                <input type="checkbox" readonly=true
                                                    checked={*foo_bravo_toggler}
                                                    style="cursor:pointer" />
                                                {" "}{i}{"-Bravo"}</span> },
                                            do_not_collapse: true,
                                            onclick: {
                                                let foo_bravo_toggler =
                                                    foo_bravo_toggler.clone();
                                                Callback::from(move |e: yew::MouseEvent|
                                                {
                                                    e.prevent_default();
                                                    log::debug!("Bravo");
                                                    foo_bravo_toggler.set(
                                                        !*foo_bravo_toggler);
                                                })
                                            }
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
                <p class="pt-3">{ "Copyright Doug Elkin ?? 2022-2023. All rights reserved." }</p>
            }}
        >
            <div class="btn-group">
                <button onclick={greeting_onclick()} class="btn btn-primary">{ "Click" }</button>
            </div>
            <div class="row mt-3">
                <Card header={ html! { <h2 class="mb-0">{ "Spinners" }</h2> } } header_theme={CardTheme::Dark}>
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
            <div class="row mt-3">
                <Card header={ html! { <h2 class="mb-0">{ "Alerts" }</h2> } } header_theme={CardTheme::Dark}>
                    <Alert>{ "info / default" }</Alert>
                    <Alert theme={AlertTheme::Primary}>{ "primary" }</Alert>
                    <Alert theme={AlertTheme::Success}
                        header={html! { <span>{ "Success!" }</span> }}>
                        { "with header" }
                    </Alert>
                    <Alert theme={AlertTheme::Warning}
                        header={html! { <em>{ "Warning!" }</em> }} />
                    <Alert theme={AlertTheme::Danger}
                        header={html! { <span>{ "Error!" }</span> }}>
                        { "You've been doing it wrong." }
                    </Alert>
                    <Alert theme={AlertTheme::Dark} gradient=true>{ "dark with gradient" }</Alert>
                    <Alert theme={AlertTheme::Light}>{ "light" }</Alert>
                    <Alert theme={AlertTheme::Secondary}>{ "secondary" }</Alert>
                    <Alert
                        theme={AlertTheme::Light}
                        class="wild-style"
                        header={html! { <span>{ "light" }</span> }}
                        >
                        { "with header and custom style" }
                    </Alert>
                    <style>{"
                        .yewstrap-alert-container.wild-style {
                            padding: 0.25em;
                            background-image: linear-gradient(to top left, black, var(--bs-red), transparent, var(--bs-blue), transparent);
                            .margin-bottom: 1em;
                        }
                        .yewstrap-alert-container.wild-style .alert {
                            margin-bottom: 0;
                        }
                    "}</style>
                    <div class="row mt-3">
                        <Alert column=true width={ColumnWidth::FromF32(3.0/4.0)}>{ "3/4 Column (all breakpoints)" }</Alert>
                        <Alert column=true width={ColumnWidth::FromF32(1.0/4.0)}>{ "1/4 Column (all breakpoints)" }</Alert>
                    </div>
                    <div class="row mt-3">
                        <Alert column=true width={ColumnWidth::ByBreakpoint(Box::new(BreakpointWidths{
                            xs: None,
                            sm: None,
                            md: Some(ColumnWidth::FromF32(3.0/4.0)),
                            lg: Some(ColumnWidth::FromF32(1.0/2.0)),
                            xl: None,
                            xxl: None,
                        }))}>{ "1/2 column at lg breakpoint, 3/4 column at md breakpoint (try resizing the window, or inspect this element to see that it's using `col-md-9` and `col-lg-6`." }</Alert>
                    </div>
                </Card>
            </div>
            <div class="row mt-3">
                <Card header={ html!{ <h2 class="mb-0">{ "Cards" }</h2> } } header_theme={CardTheme::Dark}>
                    <Card
                        header={html! { <span>{ "Outer Card" }</span> }}
                        header_theme={CardTheme::Warning}
                    >
                        <div class="row">
                            <Card>{ "\"Here is my card.\"" }</Card>
                        </div>
                        <div class="row mt-3">
                            <Card class="mt-3" column=true theme={CardTheme::Black}>{ "black theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Blue}>{ "blue theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Cyan}>{ "cyan theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Danger}>{ "danger theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Dark}>{ "dark theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Gray}>{ "gray theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::GrayDark}>{ "gray dark theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Green}>{ "green theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Indigo}>{ "indigo theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Info}>{ "info theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Light}>{ "light theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Orange}>{ "orange theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Pink}>{ "pink theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Primary}>{ "primary theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Purple}>{ "purple theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Red}>{ "red theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Secondary}>{ "secondary theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Success}>{ "success theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Teal}>{ "teal theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Warning}>{ "warning theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::White}>{ "white theme" }</Card>
                            <Card class="mt-3" column=true theme={CardTheme::Yellow}>{ "yellow theme" }</Card>
                        </div>
                        <div class="row mt-3">
                            <Card
                                column=true
                                header={html! { <span>{ "Here is a themed card with a header" }</span> }}
                                theme={CardTheme::Dark} />
                            <Card
                                column=true
                                header={html! { <span>{ "This one has a red head" }</span> }}
                                header_theme={CardTheme::Danger} />
                            <Card
                                column=true
                                header={html! { <span>{ "This one has a blue head" }</span> }}
                                header_theme={CardTheme::Primary}>
                                <h4 className="display-4">{ "Woo!" }</h4>
                            </Card>
                            <Card
                                column=true
                                body_theme={CardTheme::Indigo}
                                body_gradient=true
                                header={html! { <span>{ "This one has a green gradient head" }</span> }}
                                header_gradient=true
                                header_theme={CardTheme::Success}>
                                { "with an indigo gradient body" }
                            </Card>
                        </div>
                        <div class="row mt-3">
                            <Card
                                column=true
                                theme={CardTheme::Dark}
                                header={html! { <span>{ "This one has a yellow head" }</span> }}
                                header_theme={CardTheme::Warning}>
                                { "And a dark body and border!" }
                            </Card>
                            <Card
                                column=true
                                theme={CardTheme::Light}
                                header={html! { <span>{ "This one is light" }</span> }}
                                width={ColumnWidth::Auto}>
                                { "And automatically grows in width." }
                            </Card>
                            <Card
                                column=true
                                header_theme={CardTheme::Dark}
                                body_theme={CardTheme::Danger}
                                footer_theme={CardTheme::Dark}
                                header={html! { <span>{ "and" }</span> }}
                                footer={html! { <span>{ "has a footer" }</span> }}
                                width={ColumnWidth::ByBreakpoint(Box::new(BreakpointWidths {
                                    xs: None,
                                    sm: Some(ColumnWidth::FromF32(2.0 / 3.0)),
                                    md: Some(ColumnWidth::FromF32(1.0 / 4.0)),
                                    lg: None,
                                    xl: None,
                                    xxl: None,
                                }))}>
                                { "this one" }
                            </Card>
                        </div>
                    </Card>
                </Card>
            </div>
            <div class="row mt-3">
                <Card header={ html!{ <h2 class="mb-0">{ "Modals" }</h2> } } header_theme={CardTheme::Dark}>
                    <Modal
                        header_theme={ModalTheme::Indigo}
                        header_gradient=true
                        title={ html! { <span>{ "Modal Title" }</span> } }
                        set_show={{
                            let show_modal = show_modal.clone();
                            move |show| show_modal.set(show)
                        }}
                        show={*show_modal}
                        white_close_button=true
                        footer={html! {
                            <div class="btn-group">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    onclick={
                                        let show_modal = show_modal.clone();
                                        move |_| {
                                            show_modal.set(false);
                                        }
                                    }
                                >{ "Cancel" }</button>
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    onclick={
                                        let show_modal = show_modal.clone();
                                        move |_| {
                                            show_modal.set(false);
                                        }
                                    }
                                >{ "Save" }</button>
                            </div>
                        }}
                    >
                        { "Modal content goes here" }
                    </Modal>
                    <button type="button" class="btn btn-primary btn-lg"
                        onclick={move |_: yew::MouseEvent| show_modal.set(true)}
                    >
                        { "Show Modal" }
                    </button>
                </Card>
            </div>
        </AppDiv>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

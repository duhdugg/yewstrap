// WORK IN PROGRESS
use crate::{get_body, get_document, join_styles};
use yew::{
    classes, function_component, html, use_effect_with_deps, AttrValue, Callback, Children, Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub body_theme: Option<Theme>,
    #[prop_or(false)]
    pub body_gradient: bool,
    pub children: Option<Children>,
    pub class: Option<AttrValue>,
    #[prop_or(AttrValue::from("Close"))]
    pub close_button_text: AttrValue,
    #[prop_or(true)]
    pub close_on_backdrop_click: bool,
    #[prop_or(true)]
    pub close_on_escape: bool,
    #[prop_or(true)]
    pub focus_on_init: bool,
    pub footer: Option<yew::virtual_dom::VNode>,
    pub footer_theme: Option<Theme>,
    #[prop_or(false)]
    pub footer_gradient: bool,
    #[prop_or(Fullscreen::False)]
    pub fullscreen: Fullscreen,
    #[prop_or(false)]
    pub gradient: bool,
    pub header_theme: Option<Theme>,
    #[prop_or(false)]
    pub header_gradient: bool,
    #[prop_or(false)]
    pub hide_close_button: bool,
    pub set_show: Option<Callback<bool>>,
    #[prop_or(false)]
    pub show: bool,
    #[prop_or(Size::Medium)]
    pub size: Size,
    pub theme: Option<Theme>,
    pub title: Option<yew::virtual_dom::VNode>,
    #[prop_or(false)]
    pub white_close_button: bool,
}

#[function_component]
pub fn Modal(props: &Props) -> Html {
    {
        let show = props.show.clone();
        use_effect_with_deps(
            move |_| {
                // modal backdrop should be a child of the document body
                match show {
                    true => {
                        if let Some(backdrop) = get_modal_backdrop() {
                            backdrop.remove();
                        }
                        if let Some(body) = get_body() {
                            let document = get_document().expect("document fail");
                            let backdrop =
                                document.create_element("div").expect("create_element fail");
                            backdrop
                                .set_attribute("class", "modal-backdrop show")
                                .expect("set_attribute fail");
                            body.append_child(&backdrop).expect("append_child fail");
                        }
                    }
                    false => {
                        if let Some(backdrop) = get_modal_backdrop() {
                            let body = get_body().expect("get_body fail");
                            body.remove_child(&backdrop).expect("remove_child fail");
                        }
                    }
                }
                // cleanup backdrop on modal delete
                || {
                    if let Some(backdrop) = get_modal_backdrop() {
                        backdrop.remove();
                    }
                }
            },
            show,
        );
    }
    let hide_modal = {
        let set_show = props.set_show.clone();
        move || {
            match &set_show {
                Some(set_show) => {
                    set_show.emit(false);
                }
                None => {}
            };
        }
    };
    html! {
        <div
            class={classes! (
                "yewstrap-modal",
                "modal",
                match &props.show {
                    true => Some("show"),
                    _ => None,
                },
            )}
            role="dialog"
            tabIndex="-1"
            aria-hidden={match &props.show {
                false => Some("true"),
                _ => None,
            }}
            style={join_styles(&[("display", Some(match &props.show {
                true => "block",
                _ => "none",
            }))])}
        >
            <div
                class={classes! (
                    "modal-dialog",
                    &props.size.to_string(),
                    &props.fullscreen.to_string(),
                )}
                role="document"
            >
                <div class={classes! (
                    "modal-content",
                    "border-0",
                    match &props.theme {
                        Some(theme) => Some(theme.to_string()),
                        None => None,
                    },
                    match &props.gradient {
                        true => Some("bg-gradient"),
                        _ => None,
                    },
                )}>
                    <div class={classes!(
                        "modal-header",
                        match &props.header_theme {
                            Some(theme) => Some(theme.to_string()),
                            None => None,
                        },
                        match &props.header_gradient {
                            true => Some("bg-gradient"),
                            _ => None,
                        }
                    )}>
                        <h5 class="modal-title">{props.title.clone()}</h5>
                        { match &props.hide_close_button {
                            false => Some(html! { <button
                                type="button"
                                class={classes!(
                                    "btn-close",
                                    match &props.white_close_button {
                                        true => Some("btn-close-white"),
                                        _ => None
                                    },
                                )}
                                title={props.close_button_text.clone()}
                                onclick={move |_| {
                                    hide_modal();
                                }}
                                ></button> }),
                            true => None,
                        } }
                    </div>
                    <div class={classes!(
                        "modal-body",
                        match &props.body_theme {
                            Some(theme) => Some(theme.to_string()),
                            _ => None,
                        },
                        match &props.body_gradient {
                            true => Some("bg-gradient"),
                            _ => None,
                        }
                    )}>
                        {match props.children.as_ref() {
                            Some(children) => {
                                Some(html! { children.iter().collect::<Html>() })
                            },
                            None => None
                        }}
                    </div>
                    <div class={classes!(
                        "modal-footer",
                        match &props.footer_theme {
                            Some(theme) => Some(theme.to_string()),
                            _ => None,
                        },
                        match &props.footer_gradient {
                            true => Some("bg-gradient"),
                            _ => None,
                        }
                    )}>{props.footer.clone()}</div>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl Size {
    pub fn to_string(&self) -> String {
        String::from(match &self {
            Size::Small => "modal-sm",
            Size::Medium => "",
            Size::Large => "modal-lg",
            Size::ExtraLarge => "modal-xxl",
        })
    }
}

#[derive(PartialEq)]
pub enum Fullscreen {
    False,
    True,
    LessThanSm,
    LessThanMd,
    LessThanLg,
    LessThanXl,
    LessThanXxl,
}

impl Fullscreen {
    pub fn to_string(&self) -> String {
        String::from(match &self {
            Fullscreen::False => "",
            Fullscreen::True => "modal-fullscreen",
            Fullscreen::LessThanSm => "modal-fullscreen-sm-down",
            Fullscreen::LessThanMd => "modal-fullscreen-md-down",
            Fullscreen::LessThanLg => "modal-fullscreen-lg-down",
            Fullscreen::LessThanXl => "modal-fullscreen-xl-down",
            Fullscreen::LessThanXxl => "modal-fullscreen-xxl-down",
        })
    }
}

#[derive(PartialEq)]
pub enum Theme {
    Black,
    Blue,
    Cyan,
    Danger,
    Dark,
    Gray,
    GrayDark,
    Green,
    Indigo,
    Info,
    Light,
    Orange,
    Pink,
    Primary,
    Purple,
    Red,
    Secondary,
    Success,
    Teal,
    Warning,
    White,
    Yellow,
}

impl Theme {
    pub fn to_string(&self) -> String {
        String::from(match &self {
            Theme::Black => "yewstrap-theme-black",
            Theme::Blue => "yewstrap-theme-blue",
            Theme::Cyan => "yewstrap-theme-cyan",
            Theme::Danger => "yewstrap-theme-danger",
            Theme::Dark => "yewstrap-theme-dark",
            Theme::Gray => "yewstrap-theme-gray",
            Theme::GrayDark => "yewstrap-theme-gray-dark",
            Theme::Green => "yewstrap-theme-green",
            Theme::Indigo => "yewstrap-theme-indigo",
            Theme::Info => "yewstrap-theme-info",
            Theme::Light => "yewstrap-theme-light",
            Theme::Orange => "yewstrap-theme-orange",
            Theme::Pink => "yewstrap-theme-pink",
            Theme::Primary => "yewstrap-theme-primary",
            Theme::Purple => "yewstrap-theme-purple",
            Theme::Red => "yewstrap-theme-red",
            Theme::Secondary => "yewstrap-theme-secondary",
            Theme::Success => "yewstrap-theme-success",
            Theme::Teal => "yewstrap-theme-teal",
            Theme::Warning => "yewstrap-theme-warning",
            Theme::White => "yewstrap-theme-white",
            Theme::Yellow => "yewstrap-theme-yellow",
        })
    }
}

pub fn get_modal_backdrop() -> Option<web_sys::Element> {
    match get_body() {
        Some(body) => match body.query_selector(".modal-backdrop") {
            Ok(element) => element,
            _ => None,
        },
        _ => None,
    }
}

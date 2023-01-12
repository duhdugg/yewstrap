use crate::{BreakpointWidths, ColumnWidth};
use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub body_theme: Option<Theme>,
    #[prop_or(false)]
    pub body_gradient: bool,
    pub children: Option<Children>,
    pub class: Option<AttrValue>,
    #[prop_or(false)]
    pub column: bool,
    #[prop_or(false)]
    pub contain: bool,
    pub footer: Option<yew::virtual_dom::VNode>,
    pub footer_theme: Option<Theme>,
    #[prop_or(false)]
    pub footer_gradient: bool,
    pub header: Option<yew::virtual_dom::VNode>,
    pub header_theme: Option<Theme>,
    #[prop_or(false)]
    pub header_gradient: bool,
    pub theme: Option<Theme>,
    #[prop_or(ColumnWidth::ByBreakpoint(Box::new(BreakpointWidths {
        xs: None,
        sm: Some(ColumnWidth::Auto),
        md: None,
        lg: None,
        xl: None,
        xxl: None,
    })))]
    #[prop_or(None)]
    pub width: ColumnWidth,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "yewstrap-card-container",
            match &props.class {
                Some(class) => Some(class.to_string()),
                _ => None,
            },
            match &props.contain {
                true => Some("Container"),
                _ => None,
            },
            match &props.column {
                true => {
                    let width = &props.width;
                    let s = width.to_string();
                    Some(s)
                },
                _ => None,
            }
        )}>
        <div class={classes!(
            "card",
            match &props.theme {
                Some(theme) => Some(theme.to_string()),
                _ => None
            },
            )}>
                {match &props.header {
                    Some(header) => Some( html! {
                        <div class={classes!(
                            "card-header",
                            match &props.header_theme {
                                Some(theme) => Some(theme.to_string()),
                                _ => None
                            },
                            match &props.header_gradient {
                                true => Some("bg-gradient"),
                                _ => None
                            }
                        )}>{header.clone()}</div>
                    } ),
                    _ => None
                }}
                <div class={classes!(
                    "card-body",
                    match &props.body_theme {
                        Some(theme) => Some(theme.to_string()),
                        _ => None,
                    },
                    match &props.body_gradient {
                        true => Some("bg-gradient"),
                        _ => None
                    },
                )}>
                    {match props.children.as_ref() {
                        Some(children) => {
                            Some(html! { children.iter().collect::<Html>() })
                        },
                        None => None
                    }}
                </div>
                {match &props.footer {
                    Some(footer) => Some( html! {
                        <div class={classes!(
                            "card-footer",
                            match &props.footer_theme {
                                Some(theme) => Some(theme.to_string()),
                                _ => None
                            },
                            match &props.footer_gradient {
                                true => Some("bg-gradient"),
                                _ => None
                            }
                        )}>{footer.clone()}</div>
                    } ),
                    _ => None
                }}
            </div>
        </div>
    }
}

#[derive(PartialEq)]
pub enum Theme {
    Danger,
    Dark,
    Indigo,
    Info,
    Light,
    Primary,
    Secondary,
    Success,
    Warning,
}

impl Theme {
    pub fn to_string(&self) -> String {
        String::from(match &self {
            Theme::Danger => "yewstrap-theme-danger",
            Theme::Dark => "yewstrap-theme-dark",
            Theme::Indigo => "yewstrap-theme-indigo",
            Theme::Info => "yewstrap-theme-info",
            Theme::Light => "yewstrap-theme-light",
            Theme::Primary => "yewstrap-theme-primary",
            Theme::Secondary => "yewstrap-theme-secondary",
            Theme::Success => "yewstrap-theme-success",
            Theme::Warning => "yewstrap-theme-warning",
        })
    }
}

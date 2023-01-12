use crate::{BreakpointWidths, ColumnWidth};
use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Option<Children>,
    #[prop_or(None)]
    pub class: Option<AttrValue>,
    #[prop_or(false)]
    pub column: bool,
    #[prop_or(false)]
    pub contain: bool,
    #[prop_or(false)]
    pub gradient: bool,
    #[prop_or(None)]
    pub header: Option<yew::virtual_dom::VNode>,
    #[prop_or(Theme::Info)]
    pub theme: Theme,
    #[prop_or(ColumnWidth::ByBreakpoint(Box::new(BreakpointWidths {
        xs: None,
        sm: Some(ColumnWidth::Auto),
        md: None,
        lg: None,
        xl: None,
        xxl: None,
    })))]
    pub width: ColumnWidth,
}

#[function_component]
pub fn Alert(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "yewstrap-alert-container",
            match &props.class {
                Some(class) => Some(class.to_string()),
                _ => None,
            },
            match &props.contain {
                true => Some("container"),
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
                "alert",
                format!("alert-{}", &props.theme.to_string()),
                match &props.gradient {
                    true => Some("bg-gradient"),
                    _ => None,
                }
            )}>
                {match &props.header {
                    Some(header) => {
                        Some(html!{ <div>
                            <h4 class="alert-heading">{header.clone()}</h4>
                            <hr />
                        </div> })
                    },
                    None => None
                }}
                {match &props.children.as_ref() {
                    Some(children) => {
                        Some(html! { children.iter().collect::<Html>() })
                    },
                    None => None
                }}
            </div>
        </div>
    }
}

#[derive(PartialEq)]
pub enum Theme {
    Danger,
    Dark,
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
            Theme::Danger => "danger",
            Theme::Dark => "dark",
            Theme::Info => "info",
            Theme::Light => "light",
            Theme::Primary => "primary",
            Theme::Secondary => "secondary",
            Theme::Success => "success",
            Theme::Warning => "warning",
        })
    }
}

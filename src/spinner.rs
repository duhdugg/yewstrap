use crate::join_styles;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Option<Children>,
    #[prop_or(1.333)]
    pub children_font_size: f32, // in rem units
    #[prop_or(FlexDirection::Row)]
    pub flex_direction: FlexDirection,
    #[prop_or(2.0)]
    pub size: f32,
    #[prop_or(SpinnerType::Border)]
    pub spinner_type: SpinnerType,
}

#[function_component]
pub fn Spinner(props: &Props) -> Html {
    html! {
        <div class="d-flex"
            style={join_styles(&[
                ("flex-direction", Some(props.flex_direction.to_string())),
            ])}
            >
            <div
                class={match &props.spinner_type {
                    SpinnerType::Border => "spinner-border",
                    SpinnerType::Grow => "spinner-grow",
                }}
                role="status"
                style={join_styles(&[
                    ("width", Some(format!("{}rem", props.size.to_string()))),
                    ("height", Some(format!("{}rem", props.size.to_string()))),
                    ("margin-left", match &props.flex_direction {
                        FlexDirection::Column => Some("auto".to_string()),
                        _ => None
                    }),
                    ("margin-right", match &props.flex_direction {
                        FlexDirection::Column => Some("auto".to_string()),
                        _ => None
                    }),
                ])}
            />
            {match props.children.as_ref() {
                Some(children) => {
                    Some(html! {
                        <div
                            class="spinner-children"
                            style={join_styles(&[
                                ("font-size", Some(format!("{}rem", props.children_font_size.to_string()))),
                                ("line-height", match &props.flex_direction {
                                        FlexDirection::Row => {
                                        Some(format!("{}rem", props.size.to_string()))
                                    },
                                    _ => None,
                                }),
                                ("margin-left", match &props.flex_direction {
                                    FlexDirection::Row => Some("0.5rem".to_string()),
                                    FlexDirection::Column => Some("auto".to_string()),
                                }),
                                ("margin-right", match &props.flex_direction {
                                    FlexDirection::Column => Some("auto".to_string()),
                                    _ => None,
                                }),
                                ("margin-top", match &props.flex_direction {
                                    FlexDirection::Column => Some("0.5rem".to_string()),
                                    _ => None
                                }),
                            ])}
                            >
                            { children.iter().collect::<Html>() }
                        </div>
                    })
                },
                None => Some(html! { <div class="visually-hidden">{ "Loading..." }</div> })
            }}
        </div>
    }
}

#[derive(PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

impl FlexDirection {
    fn to_string(&self) -> String {
        String::from(match &self {
            FlexDirection::Column => "column",
            FlexDirection::Row => "row",
        })
    }
}

#[derive(PartialEq)]
pub enum SpinnerType {
    Border,
    Grow,
}

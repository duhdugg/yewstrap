use yew::{
    classes, function_component, html, use_state, AttrValue, Callback, Children, Html, NodeRef,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Option<Children>,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <div class="card">
            <div class="card-body">
                {match props.children.as_ref() {
                    Some(children) => {
                        Some(html! { children.iter().collect::<Html>() })
                    },
                    None => None
                }}
            </div>
        </div>
    }
}

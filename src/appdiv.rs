use crate::navbar::{FixedTo as NavBarFixedTo, NavBar, Props as NavBarProps};
use yew::{classes, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub header: Option<Children>,
    pub footer: Option<Children>,
    pub children: Option<Children>,
    #[prop_or(false)]
    pub fluid: bool,
    #[prop_or(None)]
    pub navbar_props: Option<NavBarProps>,
}

#[function_component]
pub fn AppDiv(props: &Props) -> Html {
    let container_class = match props.fluid {
        true => "container-fluid",
        _ => "container",
    };
    let navbar = match &props.navbar_props {
        Some(navbar_props) => {
            Some(html! {
                <NavBar
                    brand={navbar_props.brand.clone()}
                    collapsible={navbar_props.collapsible}
                    fixed_to={navbar_props.fixed_to.clone()}
                    // passed fluid prop same as props.fluid, but can be overridden
                    fluid={match navbar_props.fluid {
                        Some(fluid) => fluid,
                        None => props.fluid
                    }}
                    navitems={navbar_props.navitems.clone()}
                    toggler_position={navbar_props.toggler_position.clone()}
                />
            })
        }
        None => None,
    };
    let header = match props.header.as_ref() {
        Some(header) => Some(html! { <header>
            <div class={container_class}>
                { header.iter().collect::<Html>() }
            </div>
        </header> }),
        None => None,
    };
    let children = match props.children.as_ref() {
        Some(children) => Some(html! { <main>
            <div class={container_class}>
                { children.iter().collect::<Html>() }
            </div>
        </main> }),
        None => None,
    };
    let footer = match props.footer.as_ref() {
        Some(footer) => Some(html! { <footer>
            <div class={container_class}>
                { footer.iter().collect::<Html>() }
            </div>
        </footer> }),
        None => None,
    };
    html! {
        <div class={classes!(
            "app",
            "yewstrap-app",
            match &props.navbar_props {
                Some(navbar_props) => {
                match navbar_props.fixed_to {
                    Some(NavBarFixedTo::Top) => Some("with-topfixed-navbar"),
                    _ => None
                }
                },
                None => None
            }
            )}>
            {navbar}
            {header}
            {children}
            {footer}
        </div>
    }
}

pub mod alert;
pub mod appdiv;
pub mod card;
pub mod modal;
pub mod navbar;
pub mod navitem;
pub mod spinner;

pub fn join_styles<'a, T>(map: &[(&'a str, Option<T>)]) -> String
where
    T: core::fmt::Display,
{
    let mut v: Vec<String> = vec![];
    for (key, value) in map.iter() {
        if let Some(value) = value {
            v.push(format!("{}: {}", key, value));
        }
    }
    v.join(";")
}

#[derive(PartialEq)]
pub enum ColumnWidth {
    Auto,
    FromF32(f32),
    ByBreakpoint(Box<BreakpointWidths>),
}

impl ColumnWidth {
    pub fn to_string(&self) -> String {
        match self {
            ColumnWidth::Auto => "col".to_string(),
            ColumnWidth::FromF32(f) => {
                let size = (f * 12.0) as u8;
                format!("col-{}", size)
            }
            ColumnWidth::ByBreakpoint(widths) => widths.to_string(),
        }
    }
}

#[derive(PartialEq)]
pub struct BreakpointWidths {
    pub xs: Option<ColumnWidth>,
    pub sm: Option<ColumnWidth>,
    pub md: Option<ColumnWidth>,
    pub lg: Option<ColumnWidth>,
    pub xl: Option<ColumnWidth>,
    pub xxl: Option<ColumnWidth>,
}

impl BreakpointWidths {
    pub fn to_string(&self) -> String {
        let mut classes: Vec<String> = vec![];
        match &self.xs {
            Some(width) => {
                classes.push(match width {
                    ColumnWidth::Auto => "col".to_string(),
                    ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-xs-"),
                    ColumnWidth::ByBreakpoint(_) => "".to_string(),
                });
            }
            _ => {}
        }
        match &self.sm {
            Some(width) => classes.push(match width {
                ColumnWidth::Auto => "col-sm-auto".to_string(),
                ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-sm-"),
                ColumnWidth::ByBreakpoint(_) => "".to_string(),
            }),
            _ => {}
        }
        match &self.md {
            Some(width) => classes.push(match width {
                ColumnWidth::Auto => "col-md-auto".to_string(),
                ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-md-"),
                ColumnWidth::ByBreakpoint(_) => "".to_string(),
            }),
            _ => {}
        }
        match &self.lg {
            Some(width) => classes.push(match width {
                ColumnWidth::Auto => "col-lg-auto".to_string(),
                ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-lg-"),
                ColumnWidth::ByBreakpoint(_) => "".to_string(),
            }),
            _ => {}
        }
        match &self.xl {
            Some(width) => classes.push(match width {
                ColumnWidth::Auto => "col-xl-auto".to_string(),
                ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-xl-"),
                ColumnWidth::ByBreakpoint(_) => "".to_string(),
            }),
            _ => {}
        }
        match &self.xxl {
            Some(width) => classes.push(match width {
                ColumnWidth::Auto => "col-xxl-auto".to_string(),
                ColumnWidth::FromF32(_) => width.to_string().replace("col-", "col-xxl-"),
                ColumnWidth::ByBreakpoint(_) => "".to_string(),
            }),
            _ => {}
        }
        classes.join(" ")
    }
}

pub fn get_document() -> Option<web_sys::Document> {
    match web_sys::window() {
        Some(window) => window.document(),
        _ => None
    }
}

pub fn get_body() -> Option<web_sys::HtmlElement> {
    match get_document() {
        Some(document) => document.body(),
        _ => None
    }
}

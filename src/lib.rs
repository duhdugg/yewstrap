pub mod appdiv;
pub mod card;
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

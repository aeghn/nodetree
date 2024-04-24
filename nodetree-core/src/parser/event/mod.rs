use super::possible::PossibleScore;

pub mod reminder;
pub mod timeevent;
pub mod todoevent;

pub fn retain_parts<F>(input: &str, retain_func: F) -> Vec<&str>
where
    F: FnMut(&&str) -> bool,
{
    input
        .split(" ")
        .filter(|e| e.len() > 0)
        .filter(retain_func)
        .collect()
}

pub fn retain_not_empty_parts(input: &str) -> Vec<&str> {
    retain_parts(input, |e| !e.is_empty())
}

pub trait EventBuilder
where
    Self: Sized,
{
    fn guess(input: &str) -> Vec<(Self, PossibleScore)>;

    fn is_valid(&self) -> bool;

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self>;
    fn standard_str(&self) -> String;
}

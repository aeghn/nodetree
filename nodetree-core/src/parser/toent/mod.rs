use self::eventenum::EventEnum;

use super::possible::PossibleScore;

pub mod eventenum;
pub mod timeevent;
pub mod todoevent;

#[inline]
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

#[inline]
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

use serde::{Deserialize, Serialize};

use crate::utils::idutils;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Toent {
    id: String,
    input: String,
    event: EventEnum,
}

impl Toent {
    pub fn from_standard(input: &str) -> anyhow::Result<Toent> {
        let parts = retain_parts(input, |e| !e.is_empty());

        Ok(Toent {
            id: idutils::generate_uuid(),
            input: input.to_owned(),
            event: EventEnum::from_standard(&parts)?,
        })
    }

    pub fn guess(input: &str) -> Vec<Toent> {
        let mut guess_res = EventEnum::guess(input);
        guess_res.sort_by(|e1, e2| e1.1.cmp(&e2.1));

        let res = guess_res
            .into_iter()
            .map(|e| Toent {
                id: idutils::generate_uuid(),
                input: input.to_owned(),
                event: e.0,
            })
            .collect();

        res
    }
}

#[cfg(test)]
mod test {
    use crate::parser::toent::EventBuilder;

    use super::Toent;

    #[test]
    fn test() {
        let r = Toent::from_standard("TODO");
        println!("{:?}", r);

        let r = Toent::from_standard("2024-02-12 12:00:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Toent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Toent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Toent::from_standard("2024-02-12 12:00:00 -8:00 .*5d ,10H =10t **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12 **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Toent::from_standard(
            "2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12-12 12:00 **10d =10d",
        );
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =10m **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12:00:00 +8:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12:00:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12 12");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Toent::from_standard("2024-02-12");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Toent::guess("todo");
        println!("{:?}", r);
        let r = Toent::guess("now ..5d ,10H =10m **10d =10d");
        println!("{:?}", r);
    }
}

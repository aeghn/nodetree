use std::ops::Deref;

use self::{
    eventenum::EventEnum,
    timeevent::{
        repeater::{is_repeater_seg, is_repeater_start, Repeater},
        TimeEvent,
    },
};

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

pub struct GuessType<'a> {
    original: &'a str,
    segs: Vec<&'a str>,
}

impl<'a> From<&'a str> for GuessType<'a> {
    fn from(value: &'a str) -> Self {
        GuessType {
            original: &value,
            segs: retain_not_empty_parts(&value),
        }
    }
}

impl<'a> Deref for GuessType<'a> {
    type Target = [&'a str];

    fn deref(&self) -> &Self::Target {
        self.segs.as_slice()
    }
}

impl<'a> AsRef<str> for GuessType<'a> {
    fn as_ref(&self) -> &str {
        &self.original
    }
}

impl<'a> GuessType<'a> {
    fn full_contains_ig_case(&self, segs: &[&str]) -> bool {
        let lower = self.original.to_ascii_lowercase();
        segs.iter().any(|e| lower.contains(&e.to_lowercase()))
    }

    fn sub<F, FS, FE>(
        &self,
        mut filter: Option<F>,
        mut start_include: Option<FS>,
        mut end_exclude: Option<FE>,
    ) -> Self
    where
        F: FnMut(&str) -> bool,
        FS: FnMut(&str) -> bool,
        FE: FnMut(&str) -> bool,
    {
        let mut segs = vec![];
        let mut start = false;
        for ele in &self.segs {
            if start || start_include.as_mut().map_or(true, |f| f(ele)) {
                start = true;
            } else {
                continue;
            }

            if end_exclude.as_mut().map_or(false, |f| f(ele)) {
                break;
            }

            if filter.as_mut().map_or(true, |f| f(ele)) {
                segs.push(*ele)
            }
        }
        GuessType {
            original: self.original,
            segs,
        }
    }

    fn filter<F>(&self, mut filter: F) -> Self
    where
        F: FnMut(&str) -> bool,
    {
        GuessType {
            original: self.original,
            segs: self.segs.iter().filter(|e| filter(e)).map(|e| *e).collect(),
        }
    }

    fn groups(&self) -> (GuessType, Vec<GuessType>) {
        let (base, repeaters) = TimeEvent::sep_base_and_others(&self.segs);

        (
            GuessType {
                original: &self.original,
                segs: base,
            },
            repeaters
                .into_iter()
                .map(|e| GuessType {
                    original: &self.original,
                    segs: e,
                })
                .collect(),
        )
    }
}

pub trait EventBuilder
where
    Self: Sized,
{
    fn guess(input: &GuessType) -> Vec<(Self, PossibleScore)>;

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
        let mut guess_res = EventEnum::guess(&input.into());
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

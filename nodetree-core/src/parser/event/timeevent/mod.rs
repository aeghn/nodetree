use crate::parser::{
    event::timeevent::repeater::{is_repeater_seg, is_repeater_start},
    possible::PossibleScore,
};

use self::{repeater::Repeater, timestamp::TimeEnum};

use super::{retain_not_empty_parts, retain_parts, EventBuilder};

pub mod endconditon;
pub mod interval;
pub mod repeater;
pub mod timestamp;

fn starts_any(input: &str, anys: &[&str]) -> bool {
    anys.iter()
        .any(|e| input.starts_with(e.to_ascii_lowercase().as_str()))
}

fn equals_any(input: &str, anys: &[&str]) -> bool {
    anys.contains(&input.to_ascii_lowercase().as_str())
}

#[derive(Debug, Clone)]
pub struct TimeEvent {
    base: TimeEnum,
    repeaters: Option<Vec<Repeater>>,
}

impl From<TimeEnum> for TimeEvent {
    fn from(value: TimeEnum) -> Self {
        Self {
            base: value,
            repeaters: None,
        }
    }
}

impl TimeEvent {
    fn sep_base_and_others<'a>(segs: &[&'a str]) -> (Vec<&'a str>, Vec<Vec<&'a str>>) {
        let mut base: Vec<&str> = vec![];
        let mut others: Vec<Vec<&str>> = vec![];
        let mut sub_other: Vec<&str> = vec![];

        let mut base_end = false;
        for ele in segs {
            if !is_repeater_seg(ele) && !base_end {
                base.push(ele)
            } else {
                base_end = true;
                if is_repeater_start(ele) {
                    if !sub_other.is_empty() {
                        others.push(sub_other);
                    }
                    sub_other = vec![ele];
                } else {
                    sub_other.push(ele)
                }
            }
        }
        if !sub_other.is_empty() {
            others.push(sub_other);
        }

        (base, others)
    }
}

impl EventBuilder for TimeEvent {
    fn guess(input: &str) -> Vec<(Self, PossibleScore)> {
        let parts = retain_not_empty_parts(input);
        let (base, others) = Self::sep_base_and_others(&parts);

        let bases = TimeEnum::guess(base.join(" ").as_str());
        let guess_repeaters: Vec<Vec<(Repeater, PossibleScore)>> = others
            .into_iter()
            .map(|e| Repeater::guess(e.join(" ").as_str()))
            .collect();

        if guess_repeaters.iter().all(|v| v.is_empty()) {
            bases.into_iter().map(|(v, p)| (v.into(), p)).collect()
        } else {
            let mut repeaters = vec![];
            for ele in guess_repeaters {
                if !ele.is_empty() {
                    repeaters.push(ele[0].clone().0)
                }
            }
            bases
                .into_iter()
                .map(|(v, p)| {
                    (
                        TimeEvent {
                            base: v,
                            repeaters: Some(repeaters.clone()),
                        },
                        p,
                    )
                })
                .collect()
        }
    }

    fn is_valid(&self) -> bool {
        self.base.is_valid()
            && (self.repeaters.is_none()
                || self
                    .repeaters
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|e| e.is_valid()))
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        let (base, others) = Self::sep_base_and_others(segs);
        let repeaters = if others.is_empty() {
            None
        } else {
            let mut repeaters = vec![];
            for ele in others.iter() {
                repeaters.push(Repeater::from_standard(ele.as_slice())?)
            }
            Some(repeaters)
        };

        Ok(TimeEvent {
            base: TimeEnum::from_standard(base.as_slice())?,
            repeaters,
        })
    }

    fn standard_str(&self) -> String {
        let mut res = String::new();

        res.push_str(self.base.standard_str().as_str());

        if let Some(repeaters) = &self.repeaters {
            for rep in repeaters.iter() {
                res.push(' ');
                res.push_str(rep.standard_str().as_str());
            }
        }

        res
    }
}

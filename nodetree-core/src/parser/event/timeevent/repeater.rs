use std::{default, str::FromStr};

use strum::{AsRefStr, EnumString};

use crate::parser::{event::EventBuilder, possible::PossibleScore};

use super::{endconditon::EndCondition, interval::TimeInterval, starts_any};

#[derive(Clone, Debug, Default, EnumString, AsRefStr)]
pub enum RepeatType {
    #[default]
    #[strum(serialize = "..")]
    OnceAfter, // ,,
    #[strum(serialize = ",,")]
    OnceBegin, // ..
    #[strum(serialize = "**")]
    RepeatEvent, // **
    #[strum(serialize = ".*")]
    RepeatTodo, // .*
}

pub fn is_repeater_start(input: &str) -> bool {
    input.starts_with(",,")
        || input.starts_with("..")
        || input.starts_with("**")
        || input.starts_with(".*")
}

pub fn is_repeater_seg(input: &str) -> bool {
    is_repeater_start(input) || input.starts_with("=") || input.starts_with(",")
}

#[derive(Clone, Debug, Default)]
pub struct Repeater {
    repeat_type: RepeatType,        // ..|,,|**|.*
    interval: Option<TimeInterval>, // ..|,,|**|.*
    alert: Option<TimeInterval>,    // ,
    end_cond: Option<EndCondition>, // =
}

const TYPE_INTERVAL: i32 = 1;
const TYPE_ALERT: i32 = 2;
const TYPE_END: i32 = 3;

impl EventBuilder for Repeater {
    fn guess(input: &str) -> Vec<(Self, PossibleScore)> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        if segs.is_empty() {
            return anyhow::bail!("Unable to parse {:?}", segs);
        }

        let mut interval = None;
        let mut alert = None;
        let mut end = None;
        let mut rtype: RepeatType = Default::default();

        let mut cur: (Vec<&str>, i32) = (vec![], -1);

        let mut last: Option<(Vec<&str>, i32)> = None;

        let mut builder = |olds: &mut Option<(Vec<&str>, i32)>| -> anyhow::Result<()> {
            if let Some((segs_inner, last_type)) = olds {
                if segs_inner.is_empty() {
                    return Ok(());
                }

                if last_type == &TYPE_INTERVAL {
                    interval = Some(TimeInterval::from_standard(&segs_inner.as_slice())?)
                } else if last_type == &TYPE_ALERT {
                    alert = Some(TimeInterval::from_standard(&segs_inner.as_slice())?)
                } else if last_type == &TYPE_END {
                    end = Some(EndCondition::from_standard(&segs_inner.as_slice())?)
                }
            }
            Ok(())
        };

        for ele in segs {
            if starts_any(ele, &[",,", "..", "**", ".*"]) {
                last.replace(cur);
                cur = (vec![&ele[2..]], TYPE_INTERVAL);

                if let Ok(v) = RepeatType::from_str(&ele[..2]) {
                    rtype = v;
                }
            } else if starts_any(ele, &[","]) {
                last.replace(cur);
                cur = (vec![&ele[1..]], TYPE_ALERT);
            } else if starts_any(ele, &["="]) {
                last.replace(cur);
                cur = (vec![&ele[1..]], TYPE_END);
            } else {
                cur.0.push(ele);
            }

            builder(&mut last)?;
        }
        builder(&mut last)?;
        builder(&mut Some(cur))?;

        Ok(Repeater {
            repeat_type: rtype,
            interval: interval,
            alert: alert,
            end_cond: end,
        })
    }

    fn standard_str(&self) -> String {
        let mut res = String::new();
        res.push_str(self.repeat_type.as_ref());

        if let Some(interval) = &self.interval {
            res.push_str(interval.standard_str().as_str());
        }

        if let Some(alert) = &self.alert {
            res.push_str(" ,");
            res.push_str(alert.standard_str().as_str());
        }

        if let Some(end_cond) = &self.end_cond {
            res.push_str(" =");
            res.push_str(end_cond.standard_str().as_str());
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::parser::event::EventBuilder;

    use super::Repeater;

    #[test]
    fn test() {
        let v = Repeater::from_standard(&["..10d", ",10H", "=2t"]);
        println!("{}", v.unwrap().standard_str());
        let v = Repeater::from_standard(&["**10d", "=2022-12-23"]);
        println!("{}", v.unwrap().standard_str());
        let v = Repeater::from_standard(&[",,10d", "=3w"]);
        println!("{}", v.unwrap().standard_str());
        let v = Repeater::from_standard(&[".*20d", ",10H", "=2t"]);
        println!("{}", v.unwrap().standard_str());
    }
}

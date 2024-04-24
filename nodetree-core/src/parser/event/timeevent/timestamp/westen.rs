use std::{ops::Deref, str::FromStr, vec};

use chrono::{DateTime, Datelike, FixedOffset, Timelike, Utc};
use regex::Regex;

use crate::parser::{
    event::{retain_not_empty_parts, timeevent::equals_any, EventBuilder},
    possible::PossibleScore,
};

use super::{
    base::{convert_time_to_secs, BaseTimestamp},
    Timestamp, TimestampNow,
};

pub const CAL_TYPE: &str = "wes";

#[derive(Clone, Debug)]
pub struct WesTimestamp {
    offset: Option<FixedOffset>,
    timestamp: BaseTimestamp,
}

impl Deref for WesTimestamp {
    type Target = BaseTimestamp;

    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}

impl From<BaseTimestamp> for WesTimestamp {
    fn from(value: BaseTimestamp) -> Self {
        WesTimestamp {
            offset: None,
            timestamp: value,
        }
    }
}

impl TimestampNow for WesTimestamp {
    fn now_time() -> Self {
        let time = Utc::now().naive_local();
        WesTimestamp {
            offset: Default::default(),
            timestamp: BaseTimestamp {
                year: time.year().into(),
                month: time.month().into(),
                day: time.day().into(),
                hour: time.hour().into(),
                minute: time.minute().into(),
                second: time.second().into(),
            },
        }
    }

    fn now_date() -> Self {
        let time = Utc::now().naive_local();
        WesTimestamp {
            offset: Default::default(),
            timestamp: BaseTimestamp {
                year: time.year().into(),
                month: time.month().into(),
                day: time.day().into(),
                ..Default::default()
            },
        }
    }
}

impl EventBuilder for WesTimestamp {
    fn guess(input: &str) -> Vec<(Self, PossibleScore)> {
        let mut guessed = vec![];
        let trimmed = input.trim();

        if equals_any(
            &trimmed.to_ascii_lowercase().as_str(),
            &["t", "now", "time", "uijm", "shijian", "时间"],
        ) {
            guessed.push((WesTimestamp::now_time(), PossibleScore::Likely(100)));
        }

        if let Ok(standard) = Self::from_standard(&retain_not_empty_parts(input).as_slice()) {
            guessed.push((standard, PossibleScore::Yes(100)));
        }

        guessed
    }

    fn from_standard(standard: &[&str]) -> anyhow::Result<Self> {
        if standard.len() != 2 && standard.len() != 1 && standard.len() != 3 {
            anyhow::bail!("unable to parse westen timestamp: {:?}", standard)
        } else {
            let num_start: regex::Regex = Regex::new(r"^\d.*").unwrap();

            let mut ts_segs: Vec<&str> = vec![];
            let mut offset_seg = None;
            standard.into_iter().for_each(|e| {
                if num_start.is_match(e) {
                    ts_segs.push(e);
                } else {
                    offset_seg.replace(e);
                }
            });
            let offset = if let Some(o) = offset_seg {
                let value = convert_time_to_secs(o, super::base::TimeUnit::Minute)?;
                if o.starts_with("-") {
                    FixedOffset::west_opt(value)
                } else if o.starts_with("+") {
                    FixedOffset::east_opt(value)
                } else {
                    return anyhow::bail!("Time offset should starts with + or -.");
                }
            } else {
                None
            };
            let timestamp = BaseTimestamp::from_standard(ts_segs.as_slice())?;

            Ok(WesTimestamp { offset, timestamp })
        }
    }

    fn standard_str(&self) -> String {
        let mut base = self.timestamp.standard_str();

        if let Some(offset) = self.offset {
            base.push_str(" ");
            base.push_str(&offset.to_string());
        }

        base
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

impl Timestamp for WesTimestamp {
    fn to_wes_timestamp(&self) -> DateTime<Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        &CAL_TYPE
    }
}

#[cfg(test)]
mod test {

    use crate::parser::event::EventBuilder;

    use super::WesTimestamp;

    #[test]
    fn from_test() {
        let wes = WesTimestamp::from_standard(&["2020-12-02", "11:12:13", "+1:00"]);
        print!("{:?}", wes.unwrap().standard_str())
    }
}

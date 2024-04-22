use crate::parser::event::EventBuilder;

use super::{base::BaseTimestamp, Timestamp};

#[derive(Clone, Debug)]
pub struct ChnTimestamp {
    leap_month: bool,
    timestamp: BaseTimestamp,
}

impl Timestamp for ChnTimestamp {
    fn to_wes_timestamp(&self) -> chrono::prelude::DateTime<chrono::prelude::Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        todo!()
    }
}

impl EventBuilder for ChnTimestamp {
    fn guess(input: &str) -> Vec<(Self, crate::parser::possible::PossibleScore)> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        anyhow::bail!("todo")
    }

    fn standard_str(&self) -> String {
        todo!()
    }
}

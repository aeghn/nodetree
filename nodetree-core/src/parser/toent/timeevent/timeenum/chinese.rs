use crate::parser::toent::EventBuilder;

use super::{base::BaseTime, Timestamp};

#[derive(Clone, Debug)]
pub struct ChnTime {
    leap_month: bool,
    timestamp: BaseTime,
}

impl Timestamp for ChnTime {
    fn to_wes_timestamp(&self) -> chrono::prelude::DateTime<chrono::prelude::Utc> {
        todo!()
    }

    fn calender_type(&self) -> &'static str {
        todo!()
    }
}

impl EventBuilder for ChnTime {
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

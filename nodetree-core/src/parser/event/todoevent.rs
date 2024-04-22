use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use super::EventBuilder;

#[derive(Clone, Debug, EnumString, AsRefStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TodoEnum {
    Todo,
    Now,
    Wait,
    Done,
    Cancel,
}

impl EventBuilder for TodoEnum {
    fn guess(input: &str) -> Vec<(Self, crate::parser::possible::PossibleScore)> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        match segs.get(0) {
            Some(s) => Ok(Self::from_str(s)?),
            None => {
                anyhow::bail!("There should at least one seg to deserialize TodoEnum")
            }
        }
    }

    fn standard_str(&self) -> String {
        self.as_ref().to_string()
    }
}

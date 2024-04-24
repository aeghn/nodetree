use std::str::FromStr;

use strum::{AsRefStr, EnumIter, EnumString, IntoEnumIterator};

use crate::parser::possible::PossibleScore;

use super::EventBuilder;

#[derive(Clone, Debug, EnumString, AsRefStr, EnumIter)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TodoEnum {
    Todo,
    Doing,
    Wait,
    Done,
    Cancel,
}

impl EventBuilder for TodoEnum {
    fn guess(input: &str) -> Vec<(Self, crate::parser::possible::PossibleScore)> {
        let mut result = vec![];
        for ele in TodoEnum::iter() {
            let enum_str = ele.as_ref();
            let enum_len = enum_str.len();
            let upper_input = input.to_uppercase();

            let distance = distance::levenshtein(enum_str, &upper_input);
            if distance < enum_len {
                let mut pos = ((1. - distance as f32 / enum_len as f32) * 256.0) as u8;
                if enum_str.starts_with(&upper_input) {
                    pos = pos / 2 + 128;
                }
                if pos < 128 {
                    continue;
                }
                result.push((ele, PossibleScore::Num(pos)));
            }
        }

        result
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

#[cfg(test)]
mod test {
    use crate::parser::event::EventBuilder;

    use super::TodoEnum;

    #[test]
    fn test() {
        println!("{:?}", TodoEnum::guess("done"));
    }
}

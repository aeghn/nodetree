use std::default;

use chrono::Date;

#[derive(Default)]
pub enum LegalType {
    Chinese,
    #[default]
    Westen,
}

#[derive(Default)]
pub struct NTimestamp {
    legal_type: LegalType,
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl NTimestamp {
    pub fn wes() -> Self {
        NTimestamp {
            legal_type: LegalType::Westen,
            ..Default::default()
        }
    }

    pub fn chn() -> Self {
        NTimestamp {
            legal_type: LegalType::Chinese,
            ..Default::default()
        }
    }

    pub fn year(mut self, year: u32) -> Self {
        self.year = year;
        self
    }

    pub fn month(mut self, month: u8) -> Self {
        self.month = month;
        self
    }

    pub fn day(mut self, day: u8) -> Self {
        self.day = day;
        self
    }
    pub fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }
    pub fn minute(mut self, minute: u8) -> Self {
        self.minute = minute;
        self
    }

    pub fn second(mut self, second: u8) -> Self {
        self.second = second;
        self
    }
}

pub enum RepeaterEnum {
    FixedTime(NTimestamp),
    Week(u64),
}

pub struct Repeater {
    repeat_value: RepeaterEnum,
    alert_value: RepeaterEnum,
}

pub struct ParsedReminder {
    first_remind_time: Option<NTimestamp>,
    repeaters: Vec<Repeater>,
    end_time: Option<NTimestamp>,
}

pub struct Reminder {
    id: String,
    input: String,
    parsed: ParsedReminder,
}

impl From<&str> for Reminder {
    fn from(value: &str) -> Self {
        let input = value.trim();

        

        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn deser() {}
}

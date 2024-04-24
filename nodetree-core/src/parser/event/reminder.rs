use std::str::FromStr;

use crate::utils::idutils;

use super::{
    retain_parts,
    timeevent::{timestamp::TimeEnum, TimeEvent},
    todoevent::TodoEnum,
    EventBuilder,
};

#[derive(Clone, Debug)]
pub enum EventEnum {
    Time(TimeEvent),
    Todo(TodoEnum),
}

impl From<TimeEvent> for EventEnum {
    fn from(value: TimeEvent) -> Self {
        Self::Time(value)
    }
}

impl From<TodoEnum> for EventEnum {
    fn from(value: TodoEnum) -> Self {
        Self::Todo(value)
    }
}

impl EventBuilder for EventEnum {
    fn guess(input: &str) -> Vec<(Self, crate::parser::possible::PossibleScore)> {
        let todo_vec = TodoEnum::guess(input);
        let time_vec = TimeEvent::guess(input);

        let mut result = vec![];
        result.extend(todo_vec.into_iter().map(|(v1, v2)| (v1.into(), v2)));
        result.extend(time_vec.into_iter().map(|(v1, v2)| (v1.into(), v2)));
        result
    }

    fn is_valid(&self) -> bool {
        match self {
            EventEnum::Time(v) => v.is_valid(),
            EventEnum::Todo(v) => v.is_valid(),
        }
    }

    fn from_standard(segs: &[&str]) -> anyhow::Result<Self> {
        let event: EventEnum;
        if let Ok(v) = TodoEnum::from_standard(segs) {
            event = v.into();
        } else {
            match TimeEvent::from_standard(segs) {
                Ok(v) => {
                    event = v.into();
                }
                Err(err) => {
                    return anyhow::bail!(
                        "input {:?} could not be parsed by todo enum or time enum: {}",
                        segs,
                        err
                    );
                }
            }
        }
        Ok(event)
    }

    fn standard_str(&self) -> String {
        match self {
            EventEnum::Time(v) => v.standard_str(),
            EventEnum::Todo(v) => v.standard_str(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Reminder {
    id: String,
    input: String,
    event: EventEnum,
}

impl Reminder {
    pub fn from_standard(input: &str) -> anyhow::Result<Reminder> {
        let parts = retain_parts(input, |e| !e.is_empty());

        Ok(Reminder {
            id: idutils::generate_uuid(),
            input: input.to_owned(),
            event: EventEnum::from_standard(&parts)?,
        })
    }

    pub fn guess(input: &str) -> Vec<Reminder> {
        let mut guess_res = EventEnum::guess(input);
        guess_res.sort_by(|e1, e2| e1.1.cmp(&e2.1));

        let res = guess_res
            .into_iter()
            .map(|e| Reminder {
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
    use crate::parser::event::EventBuilder;

    use super::Reminder;

    #[test]
    fn test() {
        let r = Reminder::from_standard("TODO");
        println!("{:?}", r);

        let r = Reminder::from_standard("2024-02-12 12:00:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Reminder::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Reminder::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Reminder::from_standard("2024-02-12 12:00:00 -8:00 .*5d ,10H =10t **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12 **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Reminder::from_standard(
            "2024-02-12 12:00:00 +8:00 ..5d ,10H =2025-12-12 12:00 **10d =10d",
        );
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12:00:00 +8:00 ..5d ,10H =10m **10d =10d");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12:00:00 +8:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12:00:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12:00");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12 12");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02");
        println!("{:?}", r.unwrap().event.standard_str());
        let r = Reminder::from_standard("2024-02-12");
        println!("{:?}", r.unwrap().event.standard_str());

        let r = Reminder::guess("todo");
        println!("{:?}", r);
        let r = Reminder::guess("now ..5d ,10H =10m **10d =10d");
        println!("{:?}", r);
    }
}

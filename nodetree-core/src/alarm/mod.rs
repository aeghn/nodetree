#[derive(Clone)]
pub struct AlarmDefination {
    text: String,
    id: String,
    belong_node_id: String,
    enable: bool,
    create_time: usize,
    update_time: usize,
}

pub struct AlarmInstance {
    defi: AlarmDefination,
    unix_timestamp: usize,
    create_time: usize
}


pub trait AlarmMapper {
    fn update_or_insert_alarm(&self, node: &AlarmDefination) -> anyhow::Result<()>;
    fn delete_alarm_by_id(&self, id: &str) -> anyhow::Result<()>;
}
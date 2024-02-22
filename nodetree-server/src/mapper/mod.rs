pub mod postgre_mapper;

pub trait Mapper: NodeMapper {
    fn ensure_table_nodes(&self);
    fn ensure_table_tags(&self);
    fn ensure_table_alarm_instances(&self);
    fn ensure_table_alarm_definations(&self);

    fn ensure_tables(&self) {
        self.ensure_table_nodes();
        self.ensure_table_tags();
        self.ensure_table_alarm_definations();
        self.ensure_table_alarm_instances();
    }
}

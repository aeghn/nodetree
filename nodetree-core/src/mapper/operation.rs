pub enum OperationRecord {
    NodeMove(),
    NodeInsert(),
    NodeDelete(),
}

pub fn generate_operation_id() -> String {
    crate::utils::idutils::generate_uuid()
}

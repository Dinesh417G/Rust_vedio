use uuid::Uuid;

/// Time-sortable identifier. Every row in the database uses one of these.
pub fn new_id() -> String {
    Uuid::now_v7().to_string()
}

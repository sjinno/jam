#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    Applied(String),
    Accepted(String),
    Rejected(String),
}

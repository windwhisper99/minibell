use super::member::MemberId;

#[derive(Debug, Clone)]
pub enum AccessType {
    System,
    Member(MemberId),
    Guest,
}

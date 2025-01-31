use warg_protocol::{operator, package};

pub trait Policy {
    fn check_operator_record(&self, record: &operator::OperatorRecord) -> PolicyDecision;

    fn check_package_record(&self, record: &package::PackageRecord) -> PolicyDecision;
}

pub enum PolicyDecision {
    Accept,
    Reject,
}

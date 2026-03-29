use crate::error::ValidationError;

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

use juniper::{graphql_value, FieldError, GraphQLInputObject};

use super::Cursor;

#[derive(Debug, Default, GraphQLInputObject)]
pub struct Pagination {
    pub(crate) first: Option<i32>,
    pub(crate) after: Option<Cursor>,
    pub(crate) last: Option<i32>,
    pub(crate) before: Option<Cursor>,
}

impl Pagination {
    pub fn validate(&self) -> Result<(), FieldError> {
        match (
            (self.first, self.after.as_ref()),
            (self.last, self.before.as_ref()),
        ) {
            ((Some(first), _), _) if first < 0 => Err(FieldError::new(
                "'first' argument must be positive number",
                graphql_value!({
                    "code": "VALUE_OUT_OF_RANGE",
                    "min": 0,
                    "max": i32::MAX,
                }),
            )),
            (_, (Some(last), _)) if last < 0 => Err(FieldError::new(
                "'last' argument must be positive number",
                graphql_value!({
                    "code": "VALUE_OUT_OF_RANGE",
                    "min": 0,
                    "max": i32::MAX,
                }),
            )),
            ((Some(_), _), (Some(_), _)) => Err(FieldError::new(
                "Cannot use both 'first' and 'last'",
                graphql_value!(
                    {
                        "code": "INVALID_PARAM_COMBINATION",
                        "allowed": ["first+after", "last+before"]
                    }
                ),
            ))?,
            ((Some(_), _), (_, Some(_))) => Err(FieldError::new(
                "'first' cannot be used with 'before'",
                graphql_value!({
                    "code": "DIRECTION_CONFLICT"
                }),
            )),
            ((_, Some(_)), (Some(_), _)) => Err(FieldError::new(
                "'last' cannot be used with 'after'",
                graphql_value!({
                    "code": "DIRECTION_CONFLICT",
                }),
            )),
            _ => Ok(()),
        }
    }
    
    #[inline]
    pub fn limit(&self) -> i32 {
        self.first.or(self.last).unwrap_or(10)
    }
}

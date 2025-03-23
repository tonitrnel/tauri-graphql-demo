use crate::{graphql::scalar, utils::base64_url};
use juniper::GraphQLScalar;

#[derive(Debug, Clone, GraphQLScalar)]
#[graphql(with = cursor_scalar, parse_token(String))]
pub struct Cursor {
    pub(crate) id: scalar::ID,
    pub(crate) created_at: scalar::Timestamp,
}

mod cursor_scalar {
    use super::*;
    use juniper::{InputValue, ScalarValue, Value};

    pub(super) fn to_output<S: ScalarValue>(v: &Cursor) -> Value<S> {
        Value::Scalar(String::from(v).into())
    }
    pub(super) fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Cursor, String> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {v}"))
            .and_then(|v| match Cursor::try_from(v) {
                Ok(cursor) => Ok(cursor),
                Err(e) => Err(e.to_string()),
            })
    }
}

impl Cursor {
    pub fn new(id: scalar::ID, created_at: scalar::Timestamp) -> Self {
        Self { created_at, id }
    }
}

impl From<&Cursor> for String {
    fn from(cursor: &Cursor) -> Self {
        base64_url::encode(format!("{}:{}", cursor.created_at, cursor.id).as_bytes())
    }
}

impl<'a> TryFrom<&'a str> for Cursor {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        static ERR_MSG: &str = "Invalid cursor format";
        let bytes = base64_url::decode(value).map_err(|_| ERR_MSG)?;
        let str = std::str::from_utf8(&bytes).map_err(|_| ERR_MSG)?;
        let (created_at, id) = str.split_once(":").ok_or(ERR_MSG)?;
        Ok(Self {
            id: scalar::ID::from(id.parse::<i64>().map_err(|_| ERR_MSG)?),
            created_at: scalar::Timestamp::from(created_at.parse::<i64>().map_err(|_| ERR_MSG)?),
        })
    }
}
use crate::utils::base64_url;
use juniper::{graphql_scalar, GraphQLScalar, InputValue, ScalarValue, Value};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteTypeInfo, Database, Sqlite, Type};

#[derive(Clone, Debug, Deserialize, PartialEq, ScalarValue, Serialize)]
#[serde(untagged)]
pub enum CustomScalarValue {
    #[value(as_float, as_int)]
    Int(i32),
    #[value(as_float)]
    Float(f64),
    #[value(as_str, as_string, into_string)]
    String(String),
    #[value(as_bool)]
    Boolean(bool),
}

#[graphql_scalar]
#[graphql(
    with = integer_scalar,
    parse_token(String),
    scalar = CustomScalarValue
)]
// 在 Rust 由 i64 承载，在输出时会被转为 i32, 需要确保不会超过 i32::MAX
pub type Integer = i64;

mod integer_scalar {
    use super::*;

    pub(super) fn to_output<S: ScalarValue>(v: &Integer) -> Value<S> {
        // 直接强制转为更小的类型
        Value::Scalar((*v as i32).into())
    }
    pub(super) fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Integer, String> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `Int`, found: {v}"))
            .and_then(|s| match s.parse::<i64>() {
                Ok(i) => Ok(i),
                Err(e) => Err(format!("{e}")),
            })
    }
}

#[derive(Debug, Copy, GraphQLScalar, Clone, Eq, PartialEq, Serialize, sqlx::Type)]
#[graphql(
    with = timestamp_scalar,
    parse_token(String),
)]
pub struct Timestamp(i64);
mod timestamp_scalar {
    use super::*;
    use chrono::{DateTime, Utc};

    pub(super) fn to_output<S: ScalarValue>(v: &Timestamp) -> Value<S> {
        Value::Scalar(
            DateTime::<Utc>::from_timestamp(v.0, 0)
                .unwrap()
                .to_rfc3339()
                .into(),
        )
    }
    pub(super) fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Timestamp, String> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `Timestamp`, found {v}"))
            .and_then(|s| match DateTime::parse_from_rfc3339(s) {
                Ok(dt) => Ok(Timestamp(dt.timestamp())),
                Err(e) => Err(format!("{e}")),
            })
    }
}
impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl From<Timestamp> for i64 {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}
impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl Type<Sqlite> for Timestamp {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &<Sqlite as Database>::TypeInfo) -> bool {
        <i64 as Type<Sqlite>>::compatible(ty)
    }
}

#[derive(Debug, Copy, Hash, GraphQLScalar, Clone, Eq, PartialEq, Serialize, sqlx::Type)]
#[graphql(with = id_scalar, parse_token(String))]
pub struct ID(i64);

mod id_scalar {
    use super::*;

    pub(super) fn to_output<S: ScalarValue>(v: &ID) -> Value<S> {
        let value = base64_url::encode(&v.0.to_be_bytes());
        Value::Scalar(value.into())
    }
    pub(super) fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<ID, String> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {v}"))
            .and_then(|s| {
                match base64_url::decode(s)
                    .map_err(|e| format!("{e}"))
                    .and_then(|b| b.try_into().map_err(|_e| "Invalid byte length".to_string()))
                    .map(i64::from_be_bytes)
                {
                    Ok(v) => Ok(ID(v)),
                    Err(e) => Err(format!("Invalid ID, {e}")),
                }
            })
    }
}
impl From<i64> for ID {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl From<ID> for i64 {
    fn from(value: ID) -> Self {
        value.0
    }
}
impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl Type<Sqlite> for ID {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &<Sqlite as Database>::TypeInfo) -> bool {
        <i64 as Type<Sqlite>>::compatible(ty)
    }
}

impl PartialEq<i64> for ID {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

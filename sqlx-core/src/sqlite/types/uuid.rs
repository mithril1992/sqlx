use uuid::{adapter::Hyphenated, Uuid};

use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    sqlite::{type_info::DataType, Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    types::Type,
};

impl Type<Sqlite> for Uuid {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Text)
    }
}

impl Encode<'_, Sqlite> for Uuid {
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'_>>) -> IsNull {
        Encode::<Sqlite>::encode(self.to_simple().to_string(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Uuid {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let text = value.text()?;
        Uuid::parse_str(&text).map_err(Into::into)
    }
}

impl Type<Sqlite> for Hyphenated {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Text)
    }
}

impl Encode<'_, Sqlite> for Hyphenated {
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'_>>) -> IsNull {
        Encode::<Sqlite>::encode(self.to_string(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Hyphenated {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let text = value.text()?;
        Uuid::parse_str(text)
            .map_err(Into::into)
            .map(|u| u.to_hyphenated())
    }
}

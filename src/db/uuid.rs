use std::io::Write;

use diesel::{
    backend::Backend, deserialize, serialize, serialize::Output, types::FromSql, types::ToSql,
};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, AsExpression, FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UUID(String);

impl UUID {
    pub fn new(str: String) -> UUID {
        UUID(str)
    }
}

impl<DB> ToSql<diesel::sql_types::Text, DB> for UUID
where
    DB: Backend,
    String: ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        <String as ToSql<diesel::sql_types::Text, DB>>::to_sql(&self.0, out)
    }
}

impl<DB> FromSql<diesel::sql_types::Text, DB> for UUID
where
    DB: Backend,
    String: FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes).map(UUID)
    }
}

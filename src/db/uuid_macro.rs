macro_rules! generate_uuid_field {
    ( $name:ident ) => {
        use diesel::{
            backend::Backend, deserialize, serialize, serialize::Output, types::FromSql,
            types::ToSql, AsExpression,
        };
        use std::io::Write;

        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            AsExpression,
            FromSqlRow,
        )]
        #[sql_type = "diesel::sql_types::Text"]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn generate() -> $name {
                $name(uuid::Uuid::new_v4())
            }

            pub fn new(uuid: uuid::Uuid) -> $name {
                $name(uuid)
            }
        }

        impl<DB> ToSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: ToSql<diesel::sql_types::Text, DB>,
        {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                <String as ToSql<diesel::sql_types::Text, DB>>::to_sql(
                    &self.0.to_hyphenated().to_string(),
                    out,
                )
            }
        }

        impl<DB> FromSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: FromSql<diesel::sql_types::Text, DB>,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                let db_value_str =
                    <String as FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes)?;
                let uuid_value = uuid::Uuid::parse_str(&db_value_str)?;
                Ok($name::new(uuid_value))
            }
        }

        // TODO: UUID FromSql, ToSql for other types, like binary or very long integer?
    };
}

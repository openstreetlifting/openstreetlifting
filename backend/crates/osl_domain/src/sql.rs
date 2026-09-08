//! Postgres encoding for the enums that are stored as text.
//!
//! Every one of them lives in a `VARCHAR` column constrained by a `CHECK`, so
//! it travels as the spelling `as_str` produces and comes back through
//! `FromStr`. Doing that once here is what lets a query name the column as
//! `status as "status: AthleteStatus"` and get the enum rather than a `String`
//! nobody has checked.
//!
//! This is the only part of the crate that knows about a database. It is kept
//! in its own module so that it stays easy to see, and easy to move if
//! `osl_domain` ever has to become storage agnostic.

/// Teaches Postgres to read and write an enum as the text it is stored as.
///
/// Takes any type with an inherent `as_str(&self) -> &'static str` and a
/// `FromStr<Err = String>`. `osl_db` uses it for its own enums too, so the
/// encoding is written once rather than once per crate.
#[macro_export]
macro_rules! text_enum {
    ($($enum:ty),+ $(,)?) => {$(
        impl ::sqlx::Type<::sqlx::Postgres> for $enum {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                <str as ::sqlx::Type<::sqlx::Postgres>>::type_info()
            }

            /// The columns are `VARCHAR`, the literals in the queries are
            /// `TEXT`, and both have to decode, so compatibility is whatever
            /// `str` accepts rather than one named type.
            fn compatible(ty: &::sqlx::postgres::PgTypeInfo) -> bool {
                <str as ::sqlx::Type<::sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<'q> ::sqlx::Encode<'q, ::sqlx::Postgres> for $enum {
            fn encode_by_ref(
                &self,
                buf: &mut ::sqlx::postgres::PgArgumentBuffer,
            ) -> ::std::result::Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
                <&str as ::sqlx::Encode<'_, ::sqlx::Postgres>>::encode(self.as_str(), buf)
            }
        }

        impl<'r> ::sqlx::Decode<'r, ::sqlx::Postgres> for $enum {
            fn decode(
                value: ::sqlx::postgres::PgValueRef<'r>,
            ) -> ::std::result::Result<Self, ::sqlx::error::BoxDynError> {
                let raw = <&str as ::sqlx::Decode<'_, ::sqlx::Postgres>>::decode(value)?;

                raw.parse::<Self>()
                    .map_err(|message| {
                        ::std::boxed::Box::new($crate::ParseError::new(message))
                            as ::sqlx::error::BoxDynError
                    })
            }
        }
    )+};
}

text_enum!(
    crate::AthleteStatus,
    crate::CompetitionStatus,
    crate::Gender,
    crate::RisSource,
);

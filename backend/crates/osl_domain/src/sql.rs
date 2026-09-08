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

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

use crate::ParseError;

macro_rules! text_enum {
    ($($enum:ty),+ $(,)?) => {$(
        impl Type<Postgres> for $enum {
            fn type_info() -> PgTypeInfo {
                <str as Type<Postgres>>::type_info()
            }

            /// The columns are `VARCHAR`, the literals in the queries are
            /// `TEXT`, and both have to decode, so compatibility is whatever
            /// `str` accepts rather than one named type.
            fn compatible(ty: &PgTypeInfo) -> bool {
                <str as Type<Postgres>>::compatible(ty)
            }
        }

        impl<'q> Encode<'q, Postgres> for $enum {
            fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
                <&str as Encode<'_, Postgres>>::encode(self.as_str(), buf)
            }
        }

        impl<'r> Decode<'r, Postgres> for $enum {
            fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
                let raw = <&str as Decode<'_, Postgres>>::decode(value)?;

                raw.parse::<Self>()
                    .map_err(|message| Box::new(ParseError::new(message)) as BoxDynError)
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

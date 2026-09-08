//! Postgres decoding for the enums stored as text.

/// Reads an enum from the text in its column.
///
/// Takes any type with `FromStr<Err = String>`.
#[macro_export]
macro_rules! text_enum {
    ($($enum:ty),+ $(,)?) => {$(
        impl ::sqlx::Type<::sqlx::Postgres> for $enum {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                <str as ::sqlx::Type<::sqlx::Postgres>>::type_info()
            }

            fn compatible(ty: &::sqlx::postgres::PgTypeInfo) -> bool {
                <str as ::sqlx::Type<::sqlx::Postgres>>::compatible(ty)
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
    crate::Movement,
    crate::RisSource,
);

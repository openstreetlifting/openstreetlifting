//! Postgres encoding for the enums stored as text.

/// Reads and writes an enum as the text in its column.
///
/// Takes any type with an inherent `as_str(&self) -> &'static str` and a
/// `FromStr<Err = String>`.
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
    crate::Movement,
    crate::RisSource,
);

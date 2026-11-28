use sqlx::{Postgres, QueryBuilder};

pub(crate) fn push_name_terms_match(
    query: &mut QueryBuilder<Postgres>,
    match_key: &str,
    name: &str,
) {
    if !has_name_terms(name) {
        query.push(" FALSE ");
        return;
    }
    query.push(
        " NOT EXISTS ( SELECT 1 FROM unnest(string_to_array(trim(regexp_replace(regexp_replace(lower(unaccent(",
    );
    query.push_bind(name.to_owned());
    query.push(format!(
        ")), '[''’]', '', 'g'), '[^[:alnum:]]+', ' ', 'g')), ' ')) AS term \
         WHERE term <> '' AND strpos({match_key}, term) = 0 ) "
    ));
}

#[must_use]
pub fn has_name_terms(name: &str) -> bool {
    name.chars().any(char::is_alphanumeric)
}

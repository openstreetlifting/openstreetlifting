use sqlx::{Postgres, QueryBuilder};

pub trait Predicate {
    fn push(&self, query: &mut QueryBuilder<Postgres>);
}

#[derive(Debug, Clone)]
pub struct Filters<P>(Vec<P>);

impl<P> Default for Filters<P> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<P> FromIterator<P> for Filters<P> {
    fn from_iter<I: IntoIterator<Item = P>>(predicates: I) -> Self {
        Self(predicates.into_iter().collect())
    }
}

impl<P, const N: usize> From<[P; N]> for Filters<P> {
    fn from(predicates: [P; N]) -> Self {
        Self(predicates.into())
    }
}

impl<P> Filters<P> {
    pub fn any(&self, matches: impl Fn(&P) -> bool) -> bool {
        self.0.iter().any(matches)
    }
}

impl<P: Predicate + Clone> Filters<P> {
    pub fn push_and(&self, query: &mut QueryBuilder<Postgres>) {
        for predicate in &self.0 {
            query.push(" AND ");
            predicate.push(query);
        }
    }

    #[must_use]
    pub fn without(&self, own: impl Fn(&P) -> bool) -> Self {
        Self(
            self.0
                .iter()
                .filter(|predicate| !own(predicate))
                .cloned()
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum By {
        Country(&'static str),
        Year(i32),
    }

    impl Predicate for By {
        fn push(&self, query: &mut QueryBuilder<Postgres>) {
            match self {
                Self::Country(country) => query.push("country = ").push_bind(*country),
                Self::Year(year) => query.push("year = ").push_bind(*year),
            };
        }
    }

    fn sql(filters: &Filters<By>) -> String {
        let mut query = QueryBuilder::new("WHERE TRUE");
        filters.push_and(&mut query);
        query.into_sql().as_str().to_owned()
    }

    #[test]
    fn joins_every_predicate_with_and() {
        let filters = Filters::from([By::Country("FR"), By::Year(2025)]);
        assert_eq!(sql(&filters), "WHERE TRUE AND country = $1 AND year = $2");
        assert_eq!(sql(&Filters::default()), "WHERE TRUE");
    }

    #[test]
    fn drops_only_the_named_predicate() {
        let filters = Filters::from([By::Country("FR"), By::Year(2025)]);
        let others = filters.without(|by| matches!(by, By::Country(_)));
        assert_eq!(sql(&others), "WHERE TRUE AND year = $1");
        assert!(filters.any(|by| matches!(by, By::Country(_))));
        assert!(!others.any(|by| matches!(by, By::Country(_))));
    }
}

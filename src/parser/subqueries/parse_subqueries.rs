use crate::parser::query::Query;

pub struct SubqueryTree<'a> {
    pub root: Subquery<'a>,
}

impl<'a> SubqueryTree<'a> {
    pub fn traverse(&self) -> Vec<Query<'a>> {
        let mut queries_ordered: Vec<Query> = vec![];
        Self::traverse_inner(&self.root, &mut queries_ordered);
        queries_ordered
    }

    fn traverse_inner(sq: &Subquery<'a>, queries: &mut Vec<Query<'a>>) {
        for child in &sq.dependencies {
            Self::traverse_inner(child, queries);
        }
        queries.push(sq.query);
    }
}

pub struct Subquery<'a> {
    pub query: Query<'a>,
    pub dependencies: Vec<Subquery<'a>>,
}

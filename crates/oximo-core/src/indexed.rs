use std::ops::Index;

use oximo_expr::Expr;
use rustc_hash::FxHashMap;

use crate::set::IndexKey;

/// Sparse indexed variable: maps an `IndexKey` to a single-variable `Expr`.
///
/// Constructed by [`crate::Model::indexed_var`]. Implements [`Index`] so
/// `flow[idx]` returns the variable's expression handle directly.
#[derive(Clone, Debug)]
pub struct IndexedVar<'a> {
    pub(crate) entries: FxHashMap<IndexKey, Expr<'a>>,
}

impl<'a> IndexedVar<'a> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IndexKey, &Expr<'a>)> {
        self.entries.iter()
    }

    pub fn get<K: Into<IndexKey>>(&self, key: K) -> Option<Expr<'a>> {
        self.entries.get(&key.into()).copied()
    }
}

impl<'a, K: Into<IndexKey> + Clone> Index<K> for IndexedVar<'a> {
    type Output = Expr<'a>;
    fn index(&self, key: K) -> &Self::Output {
        self.entries.get(&key.into()).expect("IndexedVar: key not present")
    }
}

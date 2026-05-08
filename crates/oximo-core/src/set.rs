use rayon::prelude::*;

/// A finite, ordered index set.
///
/// For now we supports integer ranges and string lists.
///
/// TODO: Add tuple indexing alongside `IndexedVar` enhancements.
#[derive(Clone, Debug)]
pub enum Set {
    Range(Vec<i64>),
    Strings(Vec<String>),
}

impl Set {
    pub fn range(r: std::ops::Range<i64>) -> Self {
        Self::Range(r.collect())
    }

    pub fn from_iter_i64<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        Self::Range(iter.into_iter().collect())
    }

    pub fn strings<I, S>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Strings(iter.into_iter().map(Into::into).collect())
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Range(v) => v.len(),
            Self::Strings(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A serializable index key from a `Set`.
///
/// TODO: Add composite tuples.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IndexKey {
    Int(i64),
    Str(String),
}

impl From<i64> for IndexKey {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<&str> for IndexKey {
    fn from(s: &str) -> Self {
        Self::Str(s.to_owned())
    }
}

impl From<String> for IndexKey {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl<'a> IntoIterator for &'a Set {
    type Item = IndexKey;
    type IntoIter = SetIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Set {
    pub fn iter(&self) -> SetIter<'_> {
        SetIter { set: self, pos: 0 }
    }

    pub fn par_iter(&self) -> impl ParallelIterator<Item = IndexKey> + '_ {
        match self {
            Self::Range(v) => v.par_iter().map(|i| IndexKey::Int(*i)).collect::<Vec<_>>(),
            Self::Strings(v) => v.par_iter().map(|s| IndexKey::Str(s.clone())).collect::<Vec<_>>(),
        }
        .into_par_iter()
    }
}

#[derive(Debug)]
pub struct SetIter<'a> {
    set: &'a Set,
    pos: usize,
}

impl<'a> Iterator for SetIter<'a> {
    type Item = IndexKey;
    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.set {
            Set::Range(v) => v.get(self.pos).copied().map(IndexKey::Int),
            Set::Strings(v) => v.get(self.pos).cloned().map(IndexKey::Str),
        };
        if out.is_some() {
            self.pos += 1;
        }
        out
    }
}

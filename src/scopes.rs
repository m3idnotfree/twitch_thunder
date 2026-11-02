use std::{
    collections::{BTreeSet, btree_set},
    fmt::{Display, Formatter, Result as FmtResult},
    iter::FusedIterator,
};

use crate::Scope;

/// A set a OAuth scopes.
///
/// [`Display`] joins the scopes with a space.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Scopes {
    scopes: BTreeSet<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.scopes.clear();
        self
    }

    pub fn push(&mut self, scope: Scope) -> &mut Self {
        self.scopes.insert(scope);
        self
    }

    pub fn extend<I>(&mut self, scopes: I) -> &mut Self
    where
        I: IntoIterator<Item = Scope>,
    {
        self.scopes.extend(scopes);
        self
    }

    pub fn contains(&self, scope: Scope) -> bool {
        self.scopes.contains(&scope)
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter(self.scopes.iter())
    }

    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}

impl Display for Scopes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut scopes = self.iter();

        if let Some(first) = scopes.next() {
            f.write_str(first.as_str())?;
        }

        for scope in scopes {
            f.write_str(" ")?;
            f.write_str(scope.as_str())?;
        }

        Ok(())
    }
}

impl FromIterator<Scope> for Scopes {
    fn from_iter<T: IntoIterator<Item = Scope>>(iter: T) -> Self {
        Self {
            scopes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Scopes {
    type Item = Scope;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.scopes.into_iter())
    }
}

impl<'a> IntoIterator for &'a Scopes {
    type Item = Scope;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a>(btree_set::Iter<'a, Scope>);

impl Iterator for Iter<'_> {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Iter<'_> {}
impl FusedIterator for Iter<'_> {}

#[derive(Debug)]
pub struct IntoIter(btree_set::IntoIter<Scope>);

impl Iterator for IntoIter {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for IntoIter {}
impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use super::{Scope, Scopes};

    #[test]
    fn display_stable_order() {
        let a: Scopes = [Scope::UserWriteChat, Scope::BitsRead]
            .into_iter()
            .collect();
        let b: Scopes = [Scope::BitsRead, Scope::UserWriteChat]
            .into_iter()
            .collect();

        assert_eq!(a.to_string(), b.to_string());
        assert_eq!(a.to_string(), "bits:read user:write:chat");
        assert_eq!(Scopes::new().to_string(), "");
    }

    #[test]
    fn remove_duplicates() {
        let mut scopes = Scopes::new();
        scopes
            .push(Scope::ChannelManagePolls)
            .push(Scope::ChannelManagePolls);

        assert_eq!(scopes.len(), 1);
        assert!(scopes.contains(Scope::ChannelManagePolls));
    }

    #[test]
    fn iterators_len() {
        let scopes: Scopes = [Scope::BitsRead, Scope::UserEdit].into_iter().collect();

        assert_eq!(scopes.iter().len(), 2);
        assert_eq!((&scopes).into_iter().len(), 2);
        assert_eq!(scopes.into_iter().len(), 2);
    }
}

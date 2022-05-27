use crate::visitors::common::prelude::*;

use parcel_css::rules::document::MozDocumentRule;

impl<'a> Visitable<'a> for MozDocumentRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

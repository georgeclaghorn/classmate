use crate::visitors::common::prelude::*;

use parcel_css::rules::document::MozDocumentRule;

impl Visitable for MozDocumentRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

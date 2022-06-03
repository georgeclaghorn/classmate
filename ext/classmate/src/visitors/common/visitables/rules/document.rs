use crate::visitors::common::prelude::*;

use parcel_css::rules::document::MozDocumentRule;

impl Visitable for MozDocumentRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

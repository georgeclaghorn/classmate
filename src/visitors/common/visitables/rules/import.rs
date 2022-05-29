use crate::visitors::common::prelude::*;

use parcel_css::rules::import::ImportRule;

impl Visitable for ImportRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self)
    }
}

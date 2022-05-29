use crate::visitors::common::prelude::*;

use parcel_css::rules::import::ImportRule;

impl Visitable for ImportRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self)
    }
}

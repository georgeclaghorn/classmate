use crate::visitors::common::prelude::*;

use parcel_css::rules::import::ImportRule;

impl<'a> Visitable<'a> for ImportRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self)
    }
}

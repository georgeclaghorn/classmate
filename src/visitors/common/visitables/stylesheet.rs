use crate::visitors::common::prelude::*;

use std::cell::RefMut;
use parcel_css::stylesheet::StyleSheet;

impl<'a> Visitable<'a> for RefMut<'a, StyleSheet<'a>> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

impl<'a> Visitable<'a> for StyleSheet<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

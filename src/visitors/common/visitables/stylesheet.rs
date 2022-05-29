use crate::visitors::common::prelude::*;

use parcel_css::stylesheet::{StyleSheet, StyleAttribute};

impl<'a> Visitable<'a> for StyleSheet<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

impl<'a> Visitable<'a> for StyleAttribute<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}

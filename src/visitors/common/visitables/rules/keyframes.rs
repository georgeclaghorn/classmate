use crate::visitors::common::prelude::*;

use parcel_css::rules::keyframes::{KeyframesRule, Keyframe};

impl<'a> Visitable<'a> for KeyframesRule<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.keyframes.accept(visitor))
    }
}

impl<'a> Visitable<'a> for Keyframe<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}

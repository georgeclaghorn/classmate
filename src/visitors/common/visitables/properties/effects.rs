use crate::visitors::common::prelude::*;

use parcel_css::properties::effects::{FilterList, Filter};

impl<'a> Visitable<'a> for FilterList<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                FilterList::Filters(filters) => filters.accept(visitor),
                FilterList::None => Ok(())
            }
        })
    }
}

impl<'a> Visitable<'a> for Filter<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Filter::Url(url)      => url.accept(visitor),

                Filter::Blur(_)       |
                Filter::Brightness(_) |
                Filter::Contrast(_)   |
                Filter::Grayscale(_)  |
                Filter::HueRotate(_)  |
                Filter::Invert(_)     |
                Filter::Opacity(_)    |
                Filter::Saturate(_)   |
                Filter::Sepia(_)      |
                Filter::DropShadow(_) => Ok(())
            }
        })
    }
}

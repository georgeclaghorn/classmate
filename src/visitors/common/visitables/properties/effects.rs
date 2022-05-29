use crate::visitors::common::prelude::*;

use parcel_css::properties::effects::{FilterList, Filter};

impl Visitable for FilterList<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                FilterList::Filters(filters) => filters.accept(visitor),
                FilterList::None => Ok(())
            }
        })
    }
}

impl Visitable for Filter<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
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

use crate::visitors::common::prelude::*;

use parcel_css::properties::effects::{FilterList, Filter};

impl Visitable for FilterList<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                FilterList::Filters(filters) => filters.accept(visitor),
                FilterList::None => Ok(())
            }
        })
    }
}

impl Visitable for Filter<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
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

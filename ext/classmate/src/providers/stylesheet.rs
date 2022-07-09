use std::cell::{RefCell, Ref, RefMut};
use parcel_css::stylesheet::StyleSheet;
use owning_ref::OwningHandle;

pub trait StylesheetProvider<'a, 'o> {
    fn borrow(&self) -> Ref<'_, StyleSheet<'a, 'o>>;
    fn borrow_mut(&self) -> RefMut<'_, StyleSheet<'a, 'o>>;
}

pub struct Input {
    pub filename: Option<String>,
    pub code: String
}

type ParsedStylesheetHandle<'a, 'o> = OwningHandle<
    Box<Input>,
    Box<RefCell<StyleSheet<'a, 'o>>>
>;

pub struct ParsedStylesheetProvider<'a, 'o> {
    handle: ParsedStylesheetHandle<'a, 'o>
}

impl<'a, 'o> ParsedStylesheetProvider<'a, 'o> {
    pub fn try_new<E>(
        filename: Option<String>,
        code: String,
        parser: impl FnOnce(&'a Input) -> Result<StyleSheet<'a, 'o>, E>
    ) -> Result<ParsedStylesheetProvider<'a, 'o>, E> {
        Self::try_new_handle(filename, code, parser).map(|handle| Self { handle })
    }

    fn try_new_handle<E>(
        filename: Option<String>,
        code: String,
        parser: impl FnOnce(&'a Input) -> Result<StyleSheet<'a, 'o>, E>
    ) -> Result<ParsedStylesheetHandle<'a, 'o>, E> {
        OwningHandle::try_new(
            Box::new(Input { filename, code }),

            |input_ptr| {
                parser(unsafe { &*input_ptr })
                    .map(RefCell::new)
                    .map(Box::new)
            }
        )
    }
}

impl<'a, 'o> StylesheetProvider<'a, 'o> for ParsedStylesheetProvider<'a, 'o> {
    fn borrow(&self) -> Ref<'_, StyleSheet<'a, 'o>> {
        self.handle.borrow()
    }

    fn borrow_mut(&self) -> RefMut<'_, StyleSheet<'a, 'o>> {
        self.handle.borrow_mut()
    }
}

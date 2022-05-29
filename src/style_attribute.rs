use std::cell::{RefCell, Ref, RefMut};
use magnus::{
    define_module, current_receiver,
    Value, RModule, RClass, Module, Object, Error,
    TypedData, DataType, DataTypeFunctions,
    class::object,
    block::yield_value
};
use parcel_css::stylesheet::{MinifyOptions, PrinterOptions};
use owning_ref::OwningHandle;
use tap::Tap;
use crate::visitors::ProxyVisitor;

struct StyleAttribute<'a> {
    provider: Box<dyn StyleAttributeProvider<'a>>
}

impl<'a> StyleAttribute<'a> {
    fn parse(code: String) -> Result<Value, Error> {
        let maybe_provider =
            ParsedStyleAttributeProvider::try_new(
                code,
                parcel_css::stylesheet::StyleAttribute::parse
            );

        maybe_provider
            .map(|provider| Value::from(
                StyleAttribute { provider: Box::new(provider) }
            ))
            .map_err(|error| Error::new(
                crate::errors::parse_error(),
                error.to_string()
            ))
    }

    fn new() -> Result<StyleAttribute<'a>, Error> {
        Err(
            Error::new(
                magnus::exception::not_imp_error(),
                "not implemented"
            )
        )
    }

    fn minify(&'a self) -> Result<Value, Error> {
        self.provider.borrow_mut().minify(MinifyOptions::default());

        current_receiver()
    }

    fn proxy(&'a self) -> Result<Value, Error> {
        ProxyVisitor::new(|resource| yield_value(resource.to_string()))
            .visit_from(&mut self.provider.borrow_mut())
            .and_then(|_| current_receiver())
    }

    fn to_css(&'a self) -> Result<String, Error> {
        self.provider
            .borrow()
            .to_css(PrinterOptions::default())
            .map(|output| output.code)
            .map_err(|error| Error::new(
                crate::errors::print_error(),
                error.to_string()
            ))
    }
}

unsafe impl<'a> TypedData for StyleAttribute<'a> {
  fn class() -> RClass {
        *memoize!(
            RClass:
                object()
                    .const_get("Classmate")
                    .and_then(|module: RModule| module.const_get("StyleAttribute"))
                    .unwrap()
        )
    }

    fn data_type() -> &'static DataType {
        memoize!(
            DataType: {
                DataType::builder::<Self>("Classmate::StyleAttribute").tap_mut(|builder| {
                    builder.free_immediatly();
                    builder.size();
                }).build()
            }
        )
    }
}

impl<'a> DataTypeFunctions for StyleAttribute<'a> { }

pub trait StyleAttributeProvider<'a> {
    fn borrow(&self) -> Ref<'a, parcel_css::stylesheet::StyleAttribute>;
    fn borrow_mut(&self) -> RefMut<'a, parcel_css::stylesheet::StyleAttribute>;
}

type ParsedStyleAttributeHandle<'a> = OwningHandle<
    String,
    Box<RefCell<parcel_css::stylesheet::StyleAttribute<'a>>>
>;

struct ParsedStyleAttributeProvider<'a> {
    handle: ParsedStyleAttributeHandle<'a>
}

impl<'a> ParsedStyleAttributeProvider<'a> {
    fn try_new<P, E>(code: String, parser: P) -> Result<ParsedStyleAttributeProvider<'a>, E>
    where
        P: FnOnce(&'a str) -> Result<parcel_css::stylesheet::StyleAttribute<'a>, E>
    {
        Ok(
            ParsedStyleAttributeProvider {
                handle: OwningHandle::try_new(
                    code,

                    |code_ptr| parser(
                        unsafe { &*code_ptr }
                    ).map(|stylesheet| Box::new(RefCell::new(stylesheet)))
                )?
            }
        )
    }
}

impl<'a> StyleAttributeProvider<'a> for ParsedStyleAttributeProvider<'a> {
    fn borrow(&self) -> Ref<'a, parcel_css::stylesheet::StyleAttribute> {
        self.handle.borrow()
    }

    fn borrow_mut(&self) -> RefMut<'a, parcel_css::stylesheet::StyleAttribute> {
        self.handle.borrow_mut()
    }
}

pub fn initialize() -> Result<(), Error> {
    let module = define_module("Classmate")?;
    let class = module.define_class("StyleAttribute", object())?;

    class.define_singleton_method("parse", function!(StyleAttribute::parse, 1))?;
    class.define_singleton_method("new", function!(StyleAttribute::new, 0))?;

    class.define_method("minify", method!(StyleAttribute::minify, 0))?;

    class.define_method("proxy", method!(StyleAttribute::proxy, 0))?;

    class.define_method("to_css", method!(StyleAttribute::to_css, 0))?;
    class.define_alias("to_s", "to_css")
}

use magnus::{
    define_module, current_receiver,
    Value, RModule, RClass, Module, Object, Error,
    TypedData, DataType, DataTypeFunctions,
    class::object,
    block::yield_value
};
use parcel_css::stylesheet::{MinifyOptions, PrinterOptions};
use tap::Tap;
use crate::{
    providers::style_attribute::{StyleAttributeProvider, ParsedStyleAttributeProvider},
    visitors::ProxyVisitor
};

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
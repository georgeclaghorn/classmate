use magnus::{ExceptionClass, RModule, Module};

pub fn parse_error() -> ExceptionClass {
    get("ParseError")
}

pub fn print_error() -> ExceptionClass {
    get("PrintError")
}

fn get(name: &str) -> ExceptionClass {
    ExceptionClass::from_value(
        magnus::class::object()
            .const_get("Classmate")
            .and_then(|module: RModule| module.const_get(name))
            .unwrap()
    ).unwrap()
}

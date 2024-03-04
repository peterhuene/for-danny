use bindings::exports::demo::bar::foo::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn bar() -> String {
        bindings::demo::bar::foo::bar()
    }
}

bindings::export!(Component with_types_in bindings);

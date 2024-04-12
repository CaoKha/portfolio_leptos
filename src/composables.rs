use leptos_use::use_window;
use woothee::parser::{Parser, WootheeResult};

pub fn get_ua<'a>() -> Option<WootheeResult<'a>> {
    let parser = Parser::new();
    match use_window()
        .navigator()
        .and_then(|nav| nav.user_agent().ok())
    {
        Some(ua_str) => {
            let ua_str_heap_ref: &'a str = Box::leak(ua_str.into_boxed_str());
            // ua_str is a local variable to the closure,
            // in order to pass in a reference we need to allocate a new string
            // on the heap and take a reference to it
            parser.parse(ua_str_heap_ref)
        }
        None => None,
    }
}

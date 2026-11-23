//! A no-op telemetry sink.
//!
//! This build never sends or records telemetry. The [`event!`] macro and
//! [`send_event`] keep their signatures so call sites compile, but every
//! event is dropped where it is created.
use std::collections::HashMap;

/// A named event with free-form properties. Built only to be dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub event_type: String,
    pub event_properties: HashMap<String, serde_json::Value>,
}

/// Accepts a telemetry event and drops it.
///
/// The macro still evaluates each property expression, so call sites keep
/// their side effects and do not warn about unused values.
///
/// ```
/// # let url = "https://example.com";
/// telemetry::event!("Keymap Changed", version = "1.0.0");
/// telemetry::event!("Documentation Viewed", url, source = "Extension Upsell");
/// ```
#[macro_export]
macro_rules! event {
    ($name:expr) => {{
        let _ = &$name;
    }};
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {{
        let _ = &$name;
        $(
            let _ = &$crate::serialize_property!($key $(= $value)?);
        )+
    }};
}

#[macro_export]
macro_rules! serialize_property {
    ($key:ident) => {
        $key
    };
    ($key:ident = $value:expr) => {
        $value
    };
}

/// Drops the event. Telemetry is never sent or written.
pub fn send_event(event: Event) {
    drop(event);
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn event_evaluates_each_property_once() {
        let calls = Cell::new(0);
        let next = || {
            calls.set(calls.get() + 1);
            calls.get()
        };
        let path = "a.rs";
        crate::event!("Test Event", path, first = next(), second = next());
        assert_eq!(calls.get(), 2);
    }
}

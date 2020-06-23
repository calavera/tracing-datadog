use std::collections::HashMap;
use tracing::field::{Field, Visit};

#[derive(Default, Debug)]
#[doc(hidden)]
pub struct Visitor(pub(crate) HashMap<String, String>);

impl Visit for Visitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0
            .insert(field.name().to_string(), format!("{}", value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0
            .insert(field.name().to_string(), format!("{}", value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0
            .insert(field.name().to_string(), format!("{}", value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tera::Tera;

use crate::Parameters;

// Initialize a static Tera instance, wrapped in an Arc and Mutex, to handle template rendering.
lazy_static! {
    static ref TERA: Arc<Mutex<Tera>> = {
        let tera = Tera::default();
        Arc::new(Mutex::new(tera))
    };
}

// Renders the given `template` using the `context` provided as `Parameters`.
// Returns a `Result` with a `String` containing the rendered template or an error.
pub fn render(template: &str, context: &Parameters) -> Result<String, tera::Error> {
    // Lock the Tera instance before rendering.
    let mut tera = TERA.lock().unwrap();
    tera.render_str(template, &context.to_tera())
}

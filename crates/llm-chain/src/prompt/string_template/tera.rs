use tera::Tera;

use crate::Parameters;

// Renders the given `template` using the `context` provided as `Parameters`.
// Returns a `Result` with a `String` containing the rendered template or an error.
pub fn render(template: &str, context: &Parameters) -> Result<String, tera::Error> {
    Tera::one_off(template, &context.to_tera(), false)
}

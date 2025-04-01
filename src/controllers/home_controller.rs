use rocket_dyn_templates::{Template, context};

/// Renders the home page.
///
/// This function handles GET requests to the "/" endpoint.
/// It renders the "home/index" template with no context.
///
/// This function is invoked when the user navigates to the root
/// path of the application.
#[get("/")]
pub fn index() -> Template {
    Template::render("home/index", context! {})
}

pub mod method;

use anvil::filters;
use askama::Template;
use clap::Args;
use method::Method;

#[derive(Template, Args)]
#[template(path = "handler.fragment.rs", escape = "none")]
pub struct AxumHandler {
    pub name: String,
}

#[derive(Template, Args)]
#[template(path = "middleware.template.rs", escape = "none")]
pub struct Middleware {
    pub layer_name: String,
    pub middleware_name: String,
}

#[derive(Template, Args)]
#[template(path = "middleware_fn.template.rs", escape = "none")]
pub struct MiddlewareFn {
    pub name: String,
}

#[derive(Template, Args)]
#[template(path = "route.fragment.rs", escape = "none")]
pub struct Route {
    #[arg(value_enum)]
    pub method: Method,
    pub path: String,
    pub handler: String,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            method: Method::Get,
            path: "/index".into(),
            handler: "unimplemented!()".into(),
        }
    }
}

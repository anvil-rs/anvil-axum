use askama::Template;
use clap::{Args, ValueEnum};
use anvil::filters;

#[derive(Template, Args)]
#[template(path="handler.fragment.rs", escape="none")]
struct AxumHandler {
    name: String 
}

#[derive(Template, Args)]
#[template(path="middleware.rs", escape="none")]
struct Middleware {
    layer_name:  String,
    middleware_name: String,
}

#[derive(Template, Args)]
#[template(path="middleware_fn.rs", escape="none")]
struct MiddlewareFn{
    name: String 
}

#[derive(Template, Args)]
#[template(path="route.fragment.rs", escape="none")]
struct Route {
    #[arg(value_enum)]
    method: Method,
    path: String,
    handler: String
}

#[derive(ValueEnum, Debug, Copy)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Connect,
    Trace,
}

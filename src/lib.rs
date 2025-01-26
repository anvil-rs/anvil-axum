use askama::Template;
use anvil::filters;

#[derive(Template)]
#[template(path="handler.part.rs", ext="txt")]
struct AxumHandler<'a> {
    name: &'a str
}

#[derive(Template)]
#[template(path="middleware.rs", ext="txt")]
struct Middleware<'a> {
    layer_name: &'a str,
    middleware_name: &'a str,
}

#[derive(Template)]
#[template(path="middleware_fn.rs", ext="txt")]
struct MiddlewareFn<'a>{
    name: &'a str
}

use httpserver::Router;

pub fn some_router() -> Router {
    let mut router = Router::default();

    router.get("/some", |_, res| {
        res.send_text("Hello from second router");
    });

    router
}

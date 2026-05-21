use httpserver::Router;

const PORT: u16 = 3000;

fn main() {
    let mut router = Router::default();
    router.get("/", |_, res| {
        res.send_html("<h1>Default route</h1>");
    });
    router.get("/json", |_, res| {
        res.send_json("{\"message\": \"Hello World!\"}");
    });
    router.listen(PORT);
}

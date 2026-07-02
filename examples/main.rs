use httpserver::{Disposition, Router, Server};
use serde_json::json;

fn main() {
    let mut app = Server::create();
    let mut router = Router::default();

    router.mount_static("/static", "examples/content");

    router.get("/", |_, res| {
        res.send_html("<h1>Hello World!</h1>");
    });
    router.get("/json", |_, res| {
        res.send_json(json!({
            "message": "Hello World!"
        }));
    });
    router.get("/plik", |_, res| {
        res.send_file("examples/content/plik.txt", Disposition::Attachment)
            .unwrap();
    });
    router.get("/text", |_, res| {
        res.send_text("Message to send");
    });
    router.get("/pdf", |_, res| {
        res.send_file("examples/content/example.pdf", Disposition::Inline)
            .unwrap();
    });
    router.get("/xml", |_, res| {
        res.send_file("examples/content/index.xml", Disposition::Inline)
            .unwrap_or_else(|_| res.send_text("File not found"))
    });
    router.get("/query", |req, res| {
        if let Ok(query_params) = serde_json::to_value(&req.query) {
            res.send_json(query_params);
        } else {
            res.send_json(json!({"message": "Invalid query"}));
        }
    });

    app.mount(router);
    app.logger.set_access_file("logs/access.log");
    app.logger.set_error_file("logs/error.log");
    app.listen("0.0.0.0:3000").unwrap();
}

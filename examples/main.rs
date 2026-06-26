use httpserver::Router;
use serde_json::json;

const PORT: u16 = 3000;

fn main() {
    let mut router = Router::default();

    router.mount_static("/static", "examples/content");

    router.get("/", |_, res| {
        res.send_html("examples/content/index.html");
    });
    router.get("/json", |_, res| {
        res.send_json(json!({
            "message": "Hello World!"
        }));
    });
    router.get("/plik", |_, res| {
        res.send_attachment("examples/content/plik.txt");
    });
    router.get("/text", |_, res| {
        res.send_text("Message to send");
    });
    router.get("/pdf", |_, res| {
        res.send_file("examples/content/example.pdf");
    });
    router.get("/xml", |_, res| {
        res.send_file("examples/content/index.xml");
    });
    router.get("/query", |req, res| {
        if let Ok(query_params) = serde_json::to_value(&req.query) {
            res.send_json(query_params);
        } else {
            res.send_json(json!({"message": "Invalid query"}));
        }
    });
    router.listen(PORT);
}

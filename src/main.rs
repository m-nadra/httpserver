mod request;
use request::HttpRequest;

mod response;
use response::HttpResponse;

mod router;
use router::Router;

mod status_messages;

mod socket;

const PORT: u16 = 3000;

fn main() {
    let mut router = Router::default();
    router.get("/".to_owned(), |_, res| {
        res.send_html("<h1>Default route</h1>".to_owned());
    });
    router.get("/json".to_owned(), |_, res| {
        res.send_json("{\"message\": \"Hello World!\"}".to_owned());
    });
    router.listen(PORT);
}

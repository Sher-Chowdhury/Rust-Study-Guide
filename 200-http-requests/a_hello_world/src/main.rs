

fn main() {

    use http::{Request, Response};



let mut request = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request = request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

fn send(req: Request<()>) -> Response<()> {
    // ...
}


}

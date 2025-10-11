use tiny_http::Server;

fn main() {
    eprintln!("Creating server...");
    let server = Server::http("127.0.0.1:3000").unwrap();
    eprintln!("Server created successfully!");
    eprintln!("Listening on http://127.0.0.1:3000");
    eprintln!("About to call server.recv()...");
    
    loop {
        eprintln!("Waiting for request...");
        match server.recv() {
            Ok(request) => {
                eprintln!("Got request: {} {}", request.method(), request.url());
                let response = tiny_http::Response::from_string("Hello!");
                let _ = request.respond(response);
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }
}

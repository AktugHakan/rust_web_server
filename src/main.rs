use web_server::server::WebServer;

fn main() {
    let mut server = WebServer::launch(None, 1453, None).expect("Server couldn't start!");
    println!("Started!!!");
    server.add_route("/".to_string(), index);
    server.add_route("/about".to_string(), about);
    server.serve();
}

fn index() -> String {
    "<h1>Hello World!</h1>".to_string()
}

fn about() -> String {
    "<h1>Welcome to the About Page!</h1> 


    
    <input type='text'>"
        .to_string()
}

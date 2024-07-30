use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Result, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

const DEFAULT_404_PAGE: &str = "<h1>404 Not Found</h1>";

pub type Route = String;
pub type HTML = String;
pub struct WebServer {
    routes: HashMap<Route, fn() -> HTML>,
    //threading: bool,
    listener_socket: TcpListener,
    pub default_route_404: Option<Route>,
}

impl WebServer {
    pub fn launch(
        ip: Option<Ipv4Addr>,
        port: u16,
        //threading: bool,
        routes: Option<HashMap<Route, fn() -> HTML>>,
    ) -> Result<WebServer> {
        let ip = ip.unwrap_or(Ipv4Addr::UNSPECIFIED);
        let listener_socket = TcpListener::bind((ip, port))?;
        let routes = routes.unwrap_or_default();

        Ok(WebServer {
            routes,
            listener_socket,
            default_route_404: None,
            //threading,
        })
    }

    pub fn serve(&self) {
        for client in self.listener_socket.incoming() {
            let Ok(client) = client else {
                continue;
            };
            self.serve_to_client(client);
            //TODO: Add multi-threading support!
        }
    }

    fn serve_to_client(&self, mut client: TcpStream) {
        let route = Self::read_request(&client);
        let html = match self.routes.get(&route) {
            Some(f) => f(),
            None => match self.default_route_404.as_ref() {
                Some(default_route) => match self.routes.get(default_route) {
                    Some(f) => f(),
                    None => DEFAULT_404_PAGE.to_string(),
                },
                None => DEFAULT_404_PAGE.to_string(),
            },
        };

        Self::send_response(&mut client, &html);
    }

    pub fn add_route(&mut self, route: Route, rendering_function: fn() -> HTML) {
        self.routes.insert(route, rendering_function);
    }

    fn read_request(client: &TcpStream) -> Route {
        let mut reader = BufReader::new(client);

        let mut resp_header = String::with_capacity(100);
        reader.read_line(&mut resp_header).expect("Request broken!");

        let route = resp_header.split(' ').nth(1).unwrap().to_string();
        reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .for_each(drop); // Discard unused lines

        route
    }

    fn send_response(client: &mut TcpStream, html: &HTML) {
        let mut html = html.clone();

        html = Self::html_cleanup(html);

        let mut http_header = "HTTP/1.1 200 OK\r\n\
        Server: Zattiri Web 2K24\r\n\
        Content-Type: text/html\r\n\
        Connection: Closed\r\n\
        Content-Length: "
            .to_string();
        http_header += &html.len().to_string();
        http_header += "\r\n\r\n";

        let mut response = http_header;
        response += &html;
        response += "\r\n\r\n";

        client.write_all(response.as_bytes()).expect("Write error!");
    }

    fn html_cleanup(html: HTML) -> HTML {
        html.trim().replace(['\n', '\r'], "")
    }
}
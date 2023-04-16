use std::sync::Arc;
use std::thread;
use tiny_http::{Response, Server, Header, Method};
use std::str::FromStr;
use std::fs;

mod shopping_cart;

use crate::shopping_cart::{ShoppingCart, Product};

fn main() {
    let server = Arc::new( Server::http( "0.0.0.0:9975" ).unwrap() );

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push( thread::spawn( move || {
            for request in server.incoming_requests() {
                let mut headers = vec!["Content-type: text/html"];

                let response_text = match ( request.method(), request.url() ) {
                    ( Method::Get, "/" ) => {
                        let mut cart = ShoppingCart::new();

                        cart.add( Product::new( "Product Name", 10.0, 5.0 ) );
                        cart.add( Product::new( "Product Name 2", 100.0, 2.0 ) );
        
                        let total = cart.calculate_total();
                        let cart_html = cart.render();

                        String::from( "
                            <link rel=\"stylesheet\" href=\"/style.css\" type=\"text/css\" />
                            ".to_owned() + cart_html.as_str() + "
                        " )
                    },
                    ( Method::Get, "/style.css" ) => {
                        headers.push( "Content-type: text/css" );
                        match fs::read_to_string( "assets/style.css" ) {
                            Ok( content ) => content,
                            Err( _ ) => {
                                println!( "Style file not found" );
                                "Not found".to_string()
                            }
                        }
                    },
                    _ => {
                        "Path not found".to_string()
                    }
                };
                
                let mut response = Response::from_string( response_text );

                for header in headers {
                    let header_struct = Header::from_str( header ).unwrap();
                    response = response.with_header( header_struct );
                }

                let _ = request.respond( response );
            }
        } ) );
    }

    for h in handles {
        h.join().unwrap();
    }
}

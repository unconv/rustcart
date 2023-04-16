use std::sync::Arc;
use std::thread;
use tiny_http::{Response, Server, Header, Method};
use std::str::FromStr;
use std::fs;

mod shopping_cart;
mod shopping_cart_page;
mod shopping_cart_controller;

use crate::shopping_cart_page::ShoppingCartPage;
use crate::shopping_cart_controller::ShoppingCartController;

fn main() {
    let server = Arc::new( Server::http( "0.0.0.0:9975" ).unwrap() );

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push( thread::spawn( move || {
            for mut request in server.incoming_requests() {
                let mut headers = vec!["Content-type: text/html; charset=utf-8"];

                let mut request_body = String::new();
                _ = request.as_reader().read_to_string( &mut request_body ).unwrap();

                let response_text = match ( request.method(), request.url() ) {
                    ( Method::Get, "/" ) => {
                        ShoppingCartPage::render()
                    },
                    ( Method::Post, "/add_to_cart" ) => {
                        ShoppingCartController::add_to_cart( request_body )
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

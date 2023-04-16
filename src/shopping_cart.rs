use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::session::Session;

#[derive(Serialize, Deserialize)]
pub struct ShoppingCart {
    products: Vec<Product>
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    name: String,
    price: f64,
    quantity: f64,
}

impl ShoppingCart {
    pub fn new() -> Self {
        Self {
            products: Vec::new()
        }
    }

    pub fn to_json( &self ) -> String {
        serde_json::to_string( &self ).unwrap()
    }

    pub fn from_json( json: &str ) -> Self {
        serde_json::from_str( json ).unwrap()
    }

    pub fn get_session_filename( session_id: &str ) -> String {
        let mut filename = String::from( "sessions/" );
        filename.push_str( session_id );

        filename
    }

    pub fn load( session: &Session ) -> Self {
        let session_filename = ShoppingCart::get_session_filename( session.id.as_str() );

        let cart_json = fs::read_to_string( session_filename.as_str() ).unwrap_or( String::new() );

        if cart_json.is_empty() {
            return ShoppingCart::new();
        }

        ShoppingCart::from_json( cart_json.as_str() )
    }

    pub fn save( &self, session: &Session ) {
        let cart_json = self.to_json();

        let session_filename = ShoppingCart::get_session_filename( session.id.as_str() );

        let mut file = File::create( session_filename ).unwrap();
        _ = file.write_all( cart_json.as_bytes() );
    }

    pub fn add( &mut self, product: Product ) -> &Self {
        self.products.push( product );
        self
    }
    
    pub fn calculate_total( &self ) -> f64 {
        let mut total = 0.0;

        for product in &self.products {
            total += product.price * product.quantity;
        }

        total
    }

    pub fn render( &self ) -> String {
        let mut html = String::new();

        html.push_str( "
            <table cellpadding=\"5\" cellspacing=\"0\" class=\"cart\">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Price</th>
                        <th>Quantity</th>
                        <th>Total</th>
                    </tr>
                </thead>
                <tbody>
        " );

        for product in &self.products {
            let row_total = product.price * product.quantity;

            html.push_str( &("
                <tr>
                    <td>".to_owned() + product.name.as_str() + "</td>
                    <td>" + product.price.to_string().as_str() + "</td>
                    <td>" + product.quantity.to_string().as_str() + "</td>
                    <td>" + row_total.to_string().as_str() + "</td>
                </tr>
            ") );
        }

        html.push_str( "
            </tbody>
        </table>
        " );

        html
    }

    pub fn render_form( &self ) -> String {
        fs::read_to_string( "templates/add_to_cart_form.html" )
            .unwrap_or( "{template file not found}".to_owned() )
    }
}

impl Product {
    pub fn new( name: &str, price: f64, quantity: f64 ) -> Self {
        Self {
            name: name.to_string(),
            price: price,
            quantity: quantity,
        }
    }
}

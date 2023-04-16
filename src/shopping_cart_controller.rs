use querystring::querify;
use urlencoding::decode;
use crate::shopping_cart::ShoppingCart;
use crate::shopping_cart::Product;

pub struct ShoppingCartController;

impl ShoppingCartController {
    pub fn add_to_cart( request_body: String ) -> String {
        let post_data = querify( request_body.as_str() );
        let mut html = String::new();

        let mut name = None;
        let mut quantity: Option<f64> = None;
        let mut price: Option<f64> = None;

        let mut errors = Vec::new();

        for param in post_data {
            let key = match decode( param.0 ) {
                Ok( decoded ) => decoded.into_owned(),
                Err( _ ) => continue
            };

            let value = match decode( param.1 ) {
                Ok( decoded ) => decoded.into_owned(),
                Err( e ) => {
                    println!("Error in decoding: {}", e);
                    continue
                }
            };

            match key.as_str() {
                "name" => name = Some( value ),
                "quantity" => quantity = match value.parse() {
                    Ok( parsed ) => Some( parsed ),
                    Err( _ ) => continue
                },
                "price" => price = match value.parse() {
                    Ok( parsed ) => Some( parsed ),
                    Err( _ ) => continue
                },
                _ => continue
            }
        }

        if name.is_none() {
            errors.push( "Name is missing" );
        }/* else if name.expect( "name is valid option" ).is_empty() {
            errors.push( "Name is missing" );
        }*/

        if price.is_none() {
            errors.push( "Price is missing or invalid" );
        }

        if quantity.is_none() {
            errors.push( "Quantity is missing or invalid" );
        }

        if errors.len() > 0 {
            for error in errors {
                html.push_str( error );
            }

            return html;
        }

        let mut cart = ShoppingCart::load();

        let product = Product::new(
            name.unwrap().replace("+", " ").as_str(),
            price.unwrap(),
            quantity.unwrap()
        );

        cart.add( product );

        cart.save();

        html.push_str( "Added to cart" );

        html
    }
}

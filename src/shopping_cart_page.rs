use crate::shopping_cart::{ShoppingCart};
use crate::session::Session;

pub struct ShoppingCartPage;

impl ShoppingCartPage {
    pub fn render( session: &Session ) -> String {
        let cart = ShoppingCart::load( &session );

        let total = cart.calculate_total();
        let cart_html = cart.render();
        let form = cart.render_form();

        String::from( "
            <link rel=\"stylesheet\" href=\"/style.css\" type=\"text/css\" />
            ".to_owned() + cart_html.as_str() + "
            " + form.as_str() + "
        " )
    }
}

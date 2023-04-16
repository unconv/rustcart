use crate::shopping_cart::{ShoppingCart};

pub struct ShoppingCartPage;

impl ShoppingCartPage {
    pub fn render() -> String {
        let cart = ShoppingCart::load();

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

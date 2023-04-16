pub struct ShoppingCart {
    products: Vec<Product>
}

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

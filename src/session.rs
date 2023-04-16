use rand::{distributions::Alphanumeric, Rng};
use tiny_http::Header;

#[derive(Debug)]
pub struct Session {
    pub id: String,
}

impl Session {
    pub fn new( headers_in: &[Header], headers_out: &mut Vec<String> ) -> Session {
        let mut session_id = None;

        for header in headers_in {
            let field = header.field.as_str().as_str();

            if field == "Cookie" {
                let value = header.value.as_str().to_string();
                let parts = value.split( "RSESSION=" ).collect::<Vec<&str>>();

                let cookie_parts = match parts.get(1) {
                    Some( part ) => part.to_string(),
                    None => break
                };

                let session = cookie_parts.split( ";" ).collect::<Vec<&str>>();

                session_id = match session.get(0) {
                    Some( part ) => Some( part.to_string() ),
                    None => break
                };
            }
        }

        if session_id.is_none() {
            let id = rand::thread_rng()
                     .sample_iter( &Alphanumeric )
                     .take( 24 )
                     .map( char::from )
                     .collect::<String>();

            let mut cookie_header = String::new();
            cookie_header.push_str( "Set-Cookie: RSESSION=" );
            cookie_header.push_str( id.as_str() );
            cookie_header.push_str( ";" );

            headers_out.push( cookie_header );

            session_id = Some( id );
        }

        Self {
            id: session_id.expect( "checked with is_none above" )
        }
    }
}

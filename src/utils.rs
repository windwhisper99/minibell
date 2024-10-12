pub mod db;
pub mod discord;

pub mod header {
    use actix_web::http::header::{HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderPair};

    pub struct HxLocation(pub &'static str);

    impl TryIntoHeaderPair for HxLocation {
        type Error = InvalidHeaderValue;

        fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
            Ok((
                HeaderName::from_static("hx-location"),
                HeaderValue::from_str(&self.0)?,
            ))
        }
    }
}

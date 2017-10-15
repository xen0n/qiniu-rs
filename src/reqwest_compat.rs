#[cfg(feature = "async-api")]
pub use self::impl_async::*;


#[cfg(feature = "async-api")]
mod impl_async {
    pub use reqwest::Error;
    pub use reqwest::Method;
    pub use reqwest::header;
    pub use reqwest::unstable::async::Client;
    pub use reqwest::unstable::async::Request;
    pub use reqwest::unstable::async::Response;
}

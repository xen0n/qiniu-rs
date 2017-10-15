#[cfg(feature = "async-api")]
pub use self::impl_async::*;
#[cfg(feature = "sync-api")]
pub use self::impl_sync::*;


#[cfg(feature = "async-api")]
mod impl_async {
    pub use reqwest::Error;
    pub use reqwest::Method;
    pub use reqwest::header;
    pub use reqwest::unstable::async::Client;
    pub use reqwest::unstable::async::Request;
    pub use reqwest::unstable::async::Response;
}


#[cfg(feature = "sync-api")]
mod impl_sync {
    pub use reqwest::Error;
    pub use reqwest::Method;
    pub use reqwest::header;
    pub use reqwest::Client;
    pub use reqwest::Request;
    pub use reqwest::Response;
}

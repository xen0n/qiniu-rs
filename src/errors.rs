error_chain! {
    foreign_links {
        ReqwestError(::reqwest::Error) #[doc="Reqwest error."];
        JsonError(::serde_json::Error) #[doc="JSON error."];
        UrlParseError(::url::ParseError) #[doc="URL parsing error."];
    }
}

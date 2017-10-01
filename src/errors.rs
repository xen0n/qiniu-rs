error_chain! {
    foreign_links {
        HyperError(::hyper::Error) #[doc="Hyper error."];
        JsonError(::serde_json::Error) #[doc="JSON error."];
        UrlParseError(::url::ParseError) #[doc="URL parsing error."];
    }
}

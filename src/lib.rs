#![recursion_limit = "1024"]
#![feature(conservative_impl_trait)]

extern crate base64;
extern crate bytes;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

extern crate tokio_core;

pub mod errors;
pub mod provider;
mod request;
mod reqwest_compat;
mod sign;

pub mod storage;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use std::error::Error;

#[derive(Debug)]
pub(super) struct Client {
    client: hyper::Client<HttpConnector>,
    core: Core,
}

impl Client {
    pub fn new() -> Result<Client, Box<Error>> {
        let mut core = Core::new()?;
        let client = hyper::Client::new(&core.handle());

        Ok(Client {
            client: client,
            core: core,
        })
    }

    pub fn get(&self, uri: String) -> Result<Vec<u8>, Box<Error>> {
        let work = self.client
            .get(uri.parse()?)
            .and_then(|res| res.body().concat2().and_then(move |body| Ok(body)));
        let result = self.core.run(work)?;

        Ok(result.to_vec())
    }
}

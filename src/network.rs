use std::{ops::Deref, path::Path};

use google_sheets4 as sheets4;

use sheets4::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2, Error, Sheets,
};

use crate::app::MyError;

pub struct Connection<S> {
    hub: Sheets<S>,
}

impl<S> Deref for Connection<S> {
    type Target = Sheets<S>;
    fn deref(&self) -> &Self::Target {
        &self.hub
    }
}

impl Connection<HttpsConnector<HttpConnector>> {
    pub async fn new() -> Self {
        Self::from_credentials("credentials.json").await
    }

    pub async fn from_credentials(creds: impl AsRef<Path>) -> Self {
        let secret = oauth2::read_application_secret(creds)
            .await
            .expect("client secret could not be read");

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        );

        Connection { hub }
    }

    pub async fn get_sheet(
        &self,
        id: impl AsRef<str>,
        range: Option<impl AsRef<str>>,
    ) -> Result<Vec<Vec<String>>, ()> {
        let (_, values) = self
            .spreadsheets()
            .values_get(id.as_ref(), "A1:F5")
            .doit()
            .await
            .unwrap();
        match values.values {
            Some(values) => Ok(values),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod test {

    use sheets4::api::BatchGetValuesResponse;

    use super::*;

    #[tokio::test]
    async fn test_new_client() {
        Connection::new().await;
    }

    #[tokio::test]
    async fn test_get_sheet() {
        let con = Connection::new().await;
        let values = con
            .get_sheet(
                "1TmJIfNXwfYNox_uToEWvGyl0ZcyavG9Z68kox5WgjdA",
                Option::<&str>::None,
            )
            .await
            .unwrap();
        println!("{:?}", values);
    }
}

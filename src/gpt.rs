use async_openai::{types::CreateEditRequest, Client as OpenAiClient};

use crate::source::Source;

/// A client for the [`OpenAI`](https://openai.com/) API, with added functionality for documenting
/// purposes.
pub struct Client {
    client: OpenAiClient,
}

impl Client {
    /// Create a new client.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Client;
    ///
    /// let client = Client::new("MY_API_KEY");
    /// ```
    pub fn new(api_key: &str) -> Self {
        let client = OpenAiClient::new().with_api_key(api_key);
        Self { client }
    }

    /// Document the given source code.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{Client, Source};
    ///
    /// let mut client = Client::new();
    /// let source = Source::from_file("path/to/file.rs")
    /// let document = client.document(&source).await.unwrap();
    /// ```
    pub async fn document(&mut self, source: &Source) -> color_eyre::Result<String> {
        let request = CreateEditRequest {
            model: "code-davinci-edit-001".to_owned(),
            input: Some(source.contents.clone()),
            instruction: source.prompt.to_owned(),
            n: None,
            temperature: Some(0.3),
            top_p: None,
        };

        let response = self.client.edits().create(request).await?;
        Ok(response.choices[0].text.clone())
    }
}

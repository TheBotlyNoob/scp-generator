//! A wrapper around the Craiyon API.

use std::time::Duration;

use anyhow::Result;
use reqwest::blocking::Client;

const API_URL: &str = "https://api.craiyon.com/draw";
const IMAGE_URL: &str = "https://img.craiyon.com";
const CRAIYON_VERSION: &str = "35s5hfwn9n78gb06";

#[derive(Debug, serde::Deserialize)]
pub struct Craiyon {
    /// A list of drawn image URLs.
    pub images: Vec<String>,
}

impl Craiyon {
    /// Creates a new drawing.
    pub fn draw(prompt: &str) -> Result<Craiyon> {
        let client = Client::builder()
            .timeout(Duration::from_secs(5 * 60))
            .build()?;
        let response = client
            .post(API_URL)
            .json(
                &serde_json::json!({ "prompt": prompt, "version": CRAIYON_VERSION, "token": null }),
            )
            .send()?;
        let body = response.json()?;

        Ok(body)
    }

    /// Gets a single image from the drawings.
    ///
    /// Will return the image in the WebP format.
    pub fn image(&self) -> Result<Vec<u8>> {
        let client = Client::new();
        let response = client
            .get(format!("{IMAGE_URL}/{}", self.images[0]))
            .send()?;
        let body = response.bytes()?;

        Ok(body.to_vec())
    }
}

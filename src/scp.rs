//! A scraper for the SCP wiki.

use anyhow::{Context, Result};
use reqwest::blocking::Client;

const BASE_URL: &str = "https://scp-wiki.wikidot.com";

/// An SCP.
#[derive(Debug)]
pub struct Scp {
    /// The SCP's number.
    pub number: u16,
    /// The SCP's title.
    pub title: String,
    /// The SCP's description.
    pub description: String,
}

impl Scp {
    /// Scrapes the SCP wiki for an SCP.
    pub fn scrape(number: u16) -> Result<Self> {
        let url = format!("{}/scp-{:03}", BASE_URL, number);
        tracing::info!("scraping {}", url);
        let client = Client::new();
        let response = client.get(url).send()?;
        let body = response.text()?;
        let dom = tl::parse(&body, Default::default())?;

        let title = Self::scrape_title(&dom)?;
        let description = Self::scrape_description(&dom)?;

        Ok(Self {
            number,
            title,
            description,
        })
    }

    fn scrape_title(dom: &tl::VDom) -> Result<String> {
        let title = dom
            .get_element_by_id("page-title")
            .and_then(|node| node.get(dom.parser()))
            .context("failed to find page title")?
            .inner_text(dom.parser());

        Ok(title.into_owned())
    }

    fn scrape_description(dom: &tl::VDom) -> Result<String> {
        Ok(dom
            .get_element_by_id("page-content")
            .and_then(|node| node.get(dom.parser()))
            .and_then(|node| {
                node.find_node(dom.parser(), &mut |node| {
                    let Some(strong) = node.find_node(dom.parser(), &mut |node| {
                        node.as_tag()
                            .map(|tag| tag.name() == "strong")
                            .unwrap_or(false)
                    }) else {
                        return false;
                    };
                    strong
                        .get(dom.parser())
                        .unwrap()
                        .inner_text(dom.parser())
                        .trim()
                        == "Description:"
                })
            })
            .and_then(|node| node.get(dom.parser()))
            .context("failed to find page content")?
            .inner_text(dom.parser())
            .into_owned())
    }
}

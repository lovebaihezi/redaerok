use bevy::{prelude::*, utils::tracing};
use scraper::{Html, Selector};
use std::borrow::Cow;
use std::pin::Pin;

pub enum Cool18FetchError {
    EHttpError(String),
}

pub struct Cool18Article<'a, 'b: 'a> {
    raw_html: &'a str,
    raw_content: Option<&'b str>,
    title: &'b str,
    author: Option<&'b str>,
    main_text: Option<String>,
    // Empty links is different from None
    external_links: Option<&'b Vec<str>>,
}

impl Cool18Article<'a, 'b> {
    #[tracing::instrument(skip(raw_html))]
    fn parse_from_raw_html<'a, 'b: 'a>(raw_html: &'a str) -> Cool18Article<'a, 'b> {
        let doc = Html::parse_document(body);
        // Get title from <title> tag
        let title_selector = Selector::parse("title").unwrap();
        let title: Option<&str> = doc
            .select(&title_selector)
            .next()
            .and_then(|element_ref| element_ref.text());
        // raw_content From the <pre> tag, despite <a> <img> <font>
        let raw_content_container_selector = Selector::parse("pre").unwrap();
        let raw_content: Option<&str> = doc
            .select(&raw_content_container_selector)
            .next()
            .and_then(|element_ref| element_ref.text());
        // external links from <a> tag under first <B> tag
        let external_links_container_selector = Selector::parse("B").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        let external_links: Option<Vec<&str>> = doc
            .select(&external_links_container_selector)
            .next()
            .map(|element_ref| element_ref.select(&a_selector))
            .map(|a_tags| a_tags.flat_map(|a_tag| a_tag.attr("href")).collect());
        Self {
            raw_html,
            title,
            author: None,
            raw_content,
            main_text: None,
            external_links,
        }
    }
}

#[tracing::instrument(skip(on_done))]
pub fn fetch_uri_article(
    uri: impl ToString + std::fmt::Debug,
    on_done: impl 'static + Send + FnOnce(Result<String, Cool18FetchError>),
) -> String {
    let request = ehttp::Request::get(uri);
    info!("Start Fetching cool18 article {:#?}", uri);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        info!("Got Response of {:#?}, Try Getting HTML Body", uri);
        match result {
            Ok(res) => match res.text() {
                Some(body) => {
                    info!("Got Text from {:#?}, scraping", uri);
                }
                None => {
                    info!("Response from {:#?} got empty body", uri);
                }
            },
            Err(e) => {
                info!("Got ehttp fetch error: {:#?} of {:#?}", e, uri);
            }
        }
    });
    String::from("Hello, world!")
}

fn extract_main_text_from_pre_element<'a>(pre_element: scraper::ElementRef<'a>) -> String {}

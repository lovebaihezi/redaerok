use bevy::{prelude::*, utils::tracing};
use scraper::{Html, Selector};

pub enum Cool18FetchError {
    EHttpError(String),
}

pub struct Cool18Article<'a> {
    raw_html: scraper::Html,
    raw_content: Option<String>,
    title: Option<String>,
    author: Option<String>,
    main_text: Option<String>,
    // Empty links is different from None
    external_links: Option<Vec<&'a str>>,
}

impl<'a> Cool18Article<'a> {
    #[tracing::instrument(skip(raw_html))]
    fn parse_from_raw_html(raw_html: &'a str) -> Cool18Article<'a> {
        let doc = Html::parse_document(raw_html);
        let doc_ref = &doc;
        // Get title from <title> tag
        let title_selector = Selector::parse("title").unwrap();
        let title = doc_ref
            .select(&title_selector)
            .next()
            .map(|element_ref| element_ref.inner_html());
        // raw_content From the <pre> tag, despite <a> <img> <font>
        let raw_content_container_selector = Selector::parse("pre").unwrap();
        let raw_content = doc_ref
            .select(&raw_content_container_selector)
            .next()
            .map(|element_ref| element_ref.inner_html());
        // external links from <a> tag under first <B> tag
        let external_links_container_selector = Selector::parse("B").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        let external_links: Option<Vec<&str>> = doc_ref
            .select(&external_links_container_selector)
            .next()
            .map(|element_ref| element_ref.select(&a_selector))
            .map(|a_tags| a_tags.flat_map(|a_tag| a_tag.attr("href")).collect());
        Self {
            raw_html: doc,
            title,
            author: None,
            raw_content,
            main_text: None,
            external_links,
        }
    }
}

/// # Fetch Cool18 Article
/// It will copy the uri(to_string), and using ehttp to get the HTML then using scrpaer to get the novel content
#[tracing::instrument(skip(on_done))]
pub fn fetch_uri_article(
    uri: impl ToString + std::fmt::Debug,
    on_done: impl 'static + Send + FnOnce(Result<Cool18Article, Cool18FetchError>),
) {
    let uri = uri.to_string();
    let uri_ref = &uri;
    let request = ehttp::Request::get(uri_ref);
    info!("Start Fetching cool18 article {:#?}", uri_ref);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        info!("Got Response of {:#?}, Try Getting HTML Body", uri);
        match result {
            Ok(res) => match res.text() {
                Some(body) => {
                    info!("Got Text from {:#?}, scraping", uri);
                    let article = Cool18Article::parse_from_raw_html(&body);
                    on_done(Ok(article));
                }
                None => {
                    warn!("Response from {:#?} got empty body", uri);
                }
            },
            Err(e) => {
                error!("Got ehttp fetch error: {:#?} of {:#?}", e, uri);
            }
        }
    });
}

fn extract_main_text_from_pre_element<'a>(pre_element: scraper::ElementRef<'a>) -> String {
    _ = pre_element;
    todo!()
}

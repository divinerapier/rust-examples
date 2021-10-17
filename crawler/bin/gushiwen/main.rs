use std::time::Duration;

use crawler::Crawler;
use reqwest::blocking::{Client, ClientBuilder};
use select::predicate::Name;

#[derive(Clone)]
struct Fetcher {
    client: Client,
}

struct FetcherBuilder {
    builder: ClientBuilder,
}

impl Fetcher {
    fn builder() -> FetcherBuilder {
        FetcherBuilder {
            builder: Client::builder(),
        }
    }
}

impl FetcherBuilder {
    fn pool_max_idle_per_host(mut self, max: usize) -> Self {
        self.builder = self.builder.pool_max_idle_per_host(max);
        self
    }
    fn pool_idle_timeout<D>(mut self, timeout: D) -> Self
    where
        D: Into<Option<Duration>>,
    {
        self.builder = self.builder.pool_idle_timeout(timeout);
        self
    }
    fn build(self) -> std::result::Result<Fetcher, Box<dyn std::error::Error>> {
        Ok(Fetcher {
            client: self.builder.build()?,
        })
    }
}

impl crawler::Fetcher for Fetcher {
    fn fetch<U: AsRef<str>>(&self, u: U) -> crawler::Result<String, crawler::FetchError> {
        Ok(self.client.get(u.as_ref()).send()?.text()?)
    }
}

#[derive(Clone)]
struct Extractor;

impl crawler::Extractor for Extractor {
    fn extract<U: AsRef<str>>(
        &self,
        u: U,
        document: &str,
    ) -> crawler::Result<Option<Vec<String>>, crawler::ExtractError> {
        // println!("document: {}", document);
        let document: select::document::Document = document.into();
        let u = url::url::URL::parse(u.as_ref()).unwrap();
        let mut urls = vec![];
        for e in document.find(Name("a")) {
            if let Some(href) = e.attr("href") {
                let u = u.parse_reference(href).unwrap();
                urls.push(u.to_string());
                // println!("{} -> {}", href, u);
            }
        }
        if urls.is_empty() {
            Ok(None)
        } else {
            Ok(Some(urls))
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let fetcher = Fetcher::builder()
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(Duration::from_secs(3600))
        .build()?;

    let extractor = Extractor;

    let crawler = crawler::MultiThreadsCrawler::new(10);

    let urls = crawler.crawl("https://gushiwen.com/", fetcher, extractor)?;

    while let Ok(url) = urls.recv() {
        println!("{:?}", url);
    }

    Ok(())
}

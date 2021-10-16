use std::{
    collections::HashSet,
    sync::mpsc::{channel, Receiver, SendError, Sender},
};

mod error;

pub use error::*;

pub trait Fetcher {
    fn fetch<U: AsRef<str>>(&self, u: U) -> Result<String, FetchError>;
}

pub trait Extractor {
    fn extract<U: AsRef<str>>(
        &self,
        u: U,
        document: &str,
    ) -> Result<Option<Vec<String>>, ExtractError>;
}

pub trait Crawler {
    fn crawl<U, F, E>(
        &self,
        entrence: U,
        fetcher: F,
        extractor: E,
    ) -> Result<Receiver<Result<String>>, Error>
    where
        U: Into<String>,
        F: Fetcher + Send + 'static,
        E: Extractor + Send + 'static;
}

pub struct MultiThreadsCrawler;

impl Crawler for MultiThreadsCrawler {
    fn crawl<U, F, E>(
        &self,
        entrance: U,
        fetcher: F,
        extractor: E,
    ) -> Result<Receiver<Result<String>>>
    where
        U: Into<String>,
        F: Fetcher + Send + 'static,
        E: Extractor + Send + 'static,
    {
        let entrance = entrance.into();
        let (tx_url, rx_doc) = self.start_fetch_threads(fetcher, entrance.clone()).unwrap();
        let rx_urls = self.start_extractor_threads(extractor, rx_doc);
        self.filter_and_forward_url(entrance, tx_url, rx_urls)
    }
}

type FetchResult = (Sender<String>, Receiver<(String, Result<String, Error>)>);

pub type MultiThreadsCrawlerFetchResult = Result<FetchResult, SendError<String>>;

impl MultiThreadsCrawler {
    fn start_fetch_threads<F>(&self, fetcher: F, entrence: String) -> MultiThreadsCrawlerFetchResult
    where
        F: Fetcher + Send + 'static,
    {
        let (tx_url, rx_url) = channel();
        let (tx_doc, rx_doc) = channel();
        tx_url.send(entrence)?;
        std::thread::spawn(move || {
            while let Ok(url) = rx_url.recv() {
                if let Err(e) = tx_doc.send((url.clone(), fetcher.fetch(url).map_err(Error::Fetch)))
                {
                    println!("[FETCHER] {}", e);
                    return;
                }
            }
        });
        Ok((tx_url, rx_doc))
    }

    fn start_extractor_threads<E>(
        &self,
        extractor: E,
        rx_doc: Receiver<(String, Result<String, Error>)>,
    ) -> Receiver<Result<Option<Vec<String>>>>
    where
        E: Extractor + Send + 'static,
    {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            while let Ok((url, doc)) = rx_doc.recv() {
                if let Err(e) = tx
                    .send(doc.and_then(|doc| extractor.extract(url, &doc).map_err(Error::Extract)))
                {
                    println!("[EXTRACTOR] {}", e);
                    return;
                }
            }
        });
        rx
    }

    fn filter_and_forward_url(
        &self,
        entrence: String,
        tx_url: Sender<String>,
        rx_urls: Receiver<Result<Option<Vec<String>>>>,
    ) -> Result<Receiver<Result<String>>> {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let mut cache = HashSet::new();
            let mut inflight = 1;
            cache.insert(entrence);
            while let Ok(urls) = rx_urls.recv() {
                inflight -= 1;

                match urls {
                    Ok(urls) => {
                        if let Some(urls) = urls {
                            for url in urls {
                                if cache.insert(url.clone()) {
                                    inflight += 1;
                                    if let Err(e) = tx.send(Ok(url.clone())) {
                                        println!("[] {}", e);
                                    }
                                    if let Err(e) = tx_url.send(url) {
                                        println!("[] {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if let Err(err) = tx.send(Err(e)) {
                            println!("{}", err);
                        }
                    }
                }
                if inflight == 0 {
                    return;
                }
            }
        });

        Ok(rx)
    }
}

#[cfg(test)]
mod test {
    use crate::Crawler;

    struct Extractor {}
    struct Fetcher {}

    impl super::Fetcher for Fetcher {
        fn fetch<U: AsRef<str>>(&self, u: U) -> crate::Result<String, crate::FetchError> {
            match u.as_ref().as_bytes() {
                b"home-page" => Ok(r#"["page0", "page1", "page2"]"#.to_string()),
                b"page0" => {
                    Ok(r#"["page0", "page1", "page2", "page0-0","page0-1","page0-2"]"#.to_string())
                }
                b"page1" => {
                    Ok(r#"["page0", "page1", "page2", "page1-0","page1-1","page1-2"]"#.to_string())
                }
                b"page2" => {
                    Ok(r#"["page0", "page1", "page2", "page2-0","page2-1","page2-2"]"#.to_string())
                }
                _ => Ok(r#"[]"#.to_string()),
            }
        }
    }

    impl super::Extractor for Extractor {
        fn extract<U: AsRef<str>>(
            &self,
            _u: U,
            document: &str,
        ) -> crate::Result<Option<Vec<String>>, crate::ExtractError> {
            let urls: Vec<String> = serde_json::from_str(document).unwrap();
            if urls.is_empty() {
                Ok(None)
            } else {
                Ok(Some(urls))
            }
        }
    }

    #[test]
    fn test() {
        let crawler = super::MultiThreadsCrawler;
        let urls = crawler
            .crawl("home-page", Fetcher {}, Extractor {})
            .unwrap();

        while let Ok(url) = urls.recv() {
            println!("{:?}", url);
        }
    }
}

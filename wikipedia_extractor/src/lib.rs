// Placeholder for wikipedia_extractor crate
pub fn hello_wikipedia() {
    println!("Hello from wikipedia_extractor!");
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WikipediaArticle {
    pub title: String,
    pub url: String,
    pub revision_id: Option<u64>,
    pub content: String,
    pub links: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WikidataFact {
    pub property: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WikidataEntity {
    pub id: String,
    pub label: String,
    pub facts: Vec<WikidataFact>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_wikipedia_page() {
        let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";
        let client = reqwest::Client::builder()
            .user_agent("MyRustWikipediaExtractor/1.0 (contact@example.com)")
            .build().unwrap();
        let res = client.get(url).send().await;

        assert!(res.is_ok(), "Failed to fetch URL: {:?}", res.err());
        let body = res.unwrap().text().await;
        assert!(body.is_ok(), "Failed to get response body: {:?}", body.err());
        let content = body.unwrap();
        println!("Fetched content: {}", content);
        assert!(!content.is_empty(), "Fetched content is empty");
        assert!(content.contains("Rust"), "Content does not contain 'Rust'");
        assert!(content.contains("programming language"), "Content does not contain 'programming language'");
    }
}

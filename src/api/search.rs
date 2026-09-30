//! Search Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Search)
//!
//! - [x] search endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, QueryParams, ReturnsJsonResponse};

/// Search scope supported by Redmine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    /// Search all projects.
    All,
    /// Search only projects assigned to the current user.
    MyProject,
    /// Include subprojects of the selected project.
    Subprojects,
}

impl std::fmt::Display for SearchScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::All => "all",
            Self::MyProject => "my_project",
            Self::Subprojects => "subprojects",
        })
    }
}

/// Redmine search endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Search {
    /// Search query string.
    q: String,
    /// Search scope.
    #[builder(default)]
    scope: Option<SearchScope>,
    /// Match all query words.
    #[builder(default)]
    all_words: Option<bool>,
    /// Match titles only.
    #[builder(default)]
    titles_only: Option<bool>,
    /// Include issues.
    #[builder(default)]
    issues: Option<bool>,
    /// Include news.
    #[builder(default)]
    news: Option<bool>,
    /// Include documents.
    #[builder(default)]
    documents: Option<bool>,
    /// Include changesets.
    #[builder(default)]
    changesets: Option<bool>,
    /// Include wiki pages.
    #[builder(default)]
    wiki_pages: Option<bool>,
    /// Include messages.
    #[builder(default)]
    messages: Option<bool>,
    /// Include projects.
    #[builder(default)]
    projects: Option<bool>,
    /// Filter to open issues.
    #[builder(default)]
    open_issues: Option<bool>,
    /// Attachment search mode.
    #[builder(default)]
    attachments: Option<String>,
}

impl ReturnsJsonResponse for Search {}

impl Pageable for Search {
    fn response_wrapper_key(&self) -> String {
        "results".to_owned()
    }
}

impl Search {
    /// Create a builder for the search endpoint.
    #[must_use]
    pub fn builder() -> SearchBuilder {
        SearchBuilder::default()
    }
}

impl Endpoint for Search {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "search.json".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("q", &self.q);
        params.push_opt("scope", self.scope.as_ref().map(ToString::to_string));
        params.push_opt("all_words", self.all_words);
        params.push_opt("titles_only", self.titles_only);
        params.push_opt("issues", self.issues);
        params.push_opt("news", self.news);
        params.push_opt("documents", self.documents);
        params.push_opt("changesets", self.changesets);
        params.push_opt("wiki_pages", self.wiki_pages);
        params.push_opt("messages", self.messages);
        params.push_opt("projects", self.projects);
        params.push_opt("open_issues", self.open_issues);
        params.push_opt("attachments", self.attachments.as_ref());
        params
    }
}

/// A single result returned by the Search API.
pub type SearchResult = serde_json::Value;

#[cfg(test)]
mod test {
    use super::*;
    use url::Url;

    #[test]
    fn search_endpoint_serializes_query_and_filters() -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = Search::builder()
            .q("issue keyword".to_owned())
            .scope(SearchScope::MyProject)
            .all_words(true)
            .issues(true)
            .wiki_pages(true)
            .build()?;
        let mut url = Url::parse("https://redmine.example.com/")?.join(&endpoint.endpoint())?;
        endpoint.parameters().add_to_url(&mut url);

        assert_eq!(url.path(), "/search.json");
        let query_value = |key: &str| {
            url.query_pairs()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value.into_owned())
        };
        assert_eq!(query_value("q"), Some("issue keyword".to_owned()));
        assert_eq!(query_value("scope"), Some("my_project".to_owned()));
        assert_eq!(query_value("all_words"), Some("true".to_owned()));
        assert_eq!(query_value("issues"), Some("true".to_owned()));
        assert_eq!(query_value("wiki_pages"), Some("true".to_owned()));
        Ok(())
    }
}

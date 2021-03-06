use crate::{errors::Error, indexes::Index};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use serde_json::to_string;

// TODO support https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#matches
// TODO highlighting

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A struct containing search results and other information about the search.
pub struct SearchResults<T> {
    /// results of the query
    pub hits: Vec<T>,
    /// number of documents skipped
    pub offset: usize,
    /// number of documents to take
    pub limit: usize,
    /// total number of matches
    pub nb_hits: usize,
    /// whether nbHits is exhaustive
    pub exhaustive_nb_hits: bool,
    /// Distribution of the given facets.
    pub facets_distribution: Option<HashMap<String, HashMap<String, usize>>>,
    /// Whether facet_distribution is exhaustive
    pub exhaustive_facets_count: Option<bool>,
    /// processing time of the query
    pub processing_time_ms: usize,
    /// query originating the response
    pub query: String,
}

/// A struct representing a query.
/// You can add search parameters using the builder syntax.
/// See [here](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#query-q) for the list and description of all parameters.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::search::Query;
/// let query = Query::new("space")
///     .with_offset(42)
///     .with_limit(21);
/// ```
pub struct Query<'a> {
    /// The query parameter is the only mandatory parameter.
    /// This is the string used by the search engine to find relevant documents.
    pub query: &'a str,
    /// A number of documents to skip. If the value of the parameter offset is n, n first documents to skip. This is helpful for pagination.
    ///
    /// Example: If you want to skip the first document, set offset to 1.
    /// Default: 0
    pub offset: Option<usize>,
    /// Set a limit to the number of documents returned by search queries. If the value of the parameter limit is n, there will be n documents in the search query response. This is helpful for pagination.
    ///
    /// Example: If you want to get only two documents, set limit to 2.
    /// Default: 20
    pub limit: Option<usize>,
    /// Specify a filter to be used with the query. See the [dedicated guide](https://docs.meilisearch.com/guides/advanced_guides/filtering.html).
    pub filters: Option<&'a str>,
    /// Facet names and values to filter on. See [this page](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#facet-filters).
    pub facet_filters: Option<Vec<Vec<&'a str>>>,
    /// Facets for which to retrieve the matching count. The value `Some(None)` is the wildcard.
    pub facets_distribution: Option<Option<Vec<&'a str>>>,
    /// Attributes to display in the returned documents. Comma-separated list of attributes whose fields will be present in the returned documents.
    ///
    /// Example: If you want to get only the overview and title field and not the other fields, set `attributes_to_retrieve` to `overview,title`.
    /// Default: The [displayed attributes list](https://docs.meilisearch.com/guides/advanced_guides/settings.html#displayed-attributes) which contains by default all attributes found in the documents.
    pub attributes_to_retrieve: Option<&'a str>,
    /// TODO [doc](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#attributes-to-crop)
    pub attributes_to_crop: Option<&'a str>,
    /// Number of characters to keep on each side of the start of the matching word. See [attributes_to_crop](#structfield.attributes_to_crop).
    ///
    /// Default: 200
    pub crop_length: Option<usize>,
    /// TODO [doc](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#attributes-to-highlight)
    pub attributes_to_highlight: Option<&'a str>,
}

#[allow(missing_docs)]
impl<'a> Query<'a> {
    pub fn new(query: &'a str) -> Query<'a> {
        Query {
            query,
            offset: None,
            limit: None,
            filters: None,
            facet_filters: None,
            facets_distribution: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            attributes_to_highlight: None,
            crop_length: None,
        }
    }
    pub fn with_offset(self, offset: usize) -> Query<'a> {
        Query {
            offset: Some(offset),
            ..self
        }
    }
    pub fn with_limit(self, limit: usize) -> Query<'a> {
        Query {
            limit: Some(limit),
            ..self
        }
    }
    pub fn with_filters(self, filters: &'a str) -> Query<'a> {
        Query {
            filters: Some(filters),
            ..self
        }
    }
    pub fn with_facet_filters(self, facet_filters: Vec<Vec<&'a str>>) -> Query<'a> {
        Query {
            facet_filters: Some(facet_filters),
            ..self
        }
    }
    pub fn with_facets_distribution(self, facets_distribution: Option<Vec<&'a str>>) -> Query<'a> {
        Query {
            facets_distribution: Some(facets_distribution),
            ..self
        }
    }
    pub fn with_attributes_to_retrieve(self, attributes_to_retrieve: &'a str) -> Query<'a> {
        Query {
            attributes_to_retrieve: Some(attributes_to_retrieve),
            ..self
        }
    }
    pub fn with_attributes_to_crop(self, attributes_to_crop: &'a str) -> Query<'a> {
        Query {
            attributes_to_crop: Some(attributes_to_crop),
            ..self
        }
    }
    pub fn with_attributes_to_highlight(self, attributes_to_highlight: &'a str) -> Query<'a> {
        Query {
            attributes_to_highlight: Some(attributes_to_highlight),
            ..self
        }
    }
    pub fn with_crop_length(self, crop_length: usize) -> Query<'a> {
        Query {
            crop_length: Some(crop_length),
            ..self
        }
    }
}

impl<'a> Query<'a> {
    pub(crate) fn to_url(&self) -> String {
        use urlencoding::encode;
        let mut url = format!("?q={}", encode(self.query));

        if let Some(offset) = self.offset {
            url.push_str("&offset=");
            url.push_str(offset.to_string().as_str());
        }
        if let Some(limit) = self.limit {
            url.push_str("&limit=");
            url.push_str(limit.to_string().as_str());
        }
        if let Some(filters) = self.filters {
            url.push_str("&filters=");
            url.push_str(encode(filters).as_str());
        }
        if let Some(facet_filters) = &self.facet_filters {
            url.push_str("&facetFilters=");
            url.push_str(encode(&to_string(&facet_filters).unwrap()).as_str());
        }
        if let Some(facets_distribution) = &self.facets_distribution {
            url.push_str("&facetsDistribution=");
            match facets_distribution {
                Some(facets_distribution) => url.push_str(encode(&to_string(&facets_distribution).unwrap()).as_str()),
                None => url.push_str("*")
            }
        }
        if let Some(attributes_to_retrieve) = self.attributes_to_retrieve {
            url.push_str("&attributesToRetrieve=");
            url.push_str(encode(attributes_to_retrieve).as_str());
        }
        if let Some(attributes_to_crop) = self.attributes_to_crop {
            url.push_str("&attributesToCrop=");
            url.push_str(encode(attributes_to_crop).as_str());
        }
        if let Some(crop_length) = self.crop_length {
            url.push_str("&cropLength=");
            url.push_str(crop_length.to_string().as_str());
        }
        if let Some(attributes_to_highlight) = self.attributes_to_highlight {
            url.push_str("&attributesToHighlight=");
            url.push_str(encode(attributes_to_highlight).as_str());
        }

        url
    }

    /// Alias for [the Index method](../indexes/struct.Index.html#method.search).
    pub async fn execute<T: 'static + DeserializeOwned>(
        &'a self,
        index: &Index<'a>,
    ) -> Result<SearchResults<T>, Error> {
        index.search::<T>(&self).await
    }
}

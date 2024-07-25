use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Body {
    pub query: String,
    #[serde(rename = "facetFilters")]
    pub facet_filters: Vec<Vec<&'static str>>,
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: u8,
    #[serde(rename = "attributesToRetrieve")]
    pub attributes_to_retrieve: Vec<&'static str>,
    pub page: u8,
}

use serde::{Deserialize, Serialize};

/// Enum to be used for sorting the return
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum Order {
    /// Ascending order
    ASC,
    /// Descending order
    DESC,
}

/// Query to show the number of items per page to fetch and the which page to show
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PageQuery {
    /// Number of items per page
    pub page_size: u64,
    /// Which page to fetch
    pub page_num: u64,
}

// external types for routes, etc... (basically made so they have no generics and we can convert
// properlly)

/// Query related to vectors and arrays
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VecQueryWeb {
    /// The column to be comparing against
    pub column: String,
    /// Matches any of items specified
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub any_of: Option<Vec<serde_json::Value>>,
    /// Does NOT match any of items specified
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub none_of: Option<Vec<serde_json::Value>>,
}

/// Query for ordering the output
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct OrderQueryWeb {
    /// The column to sort
    pub column: String,
    /// What order to follow (ASC/DESC)
    pub order: Order,
}

/// Query related to ranges
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RangeQueryWeb {
    /// The column to be comparing against
    pub column: String,
    /// Strictly greater than
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub greater_than: Option<serde_json::Value>,
    /// Greater than or equal to
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub greater_than_or_equal: Option<serde_json::Value>,
    /// Strictly less than
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub less_than: Option<serde_json::Value>,
    /// Less than or equal to
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub less_than_or_equal: Option<serde_json::Value>,
}

/// Query related to strings
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct StringQueryWeb {
    /// The columb to be comparing against
    pub column: String,
    /// 'like' syntax, matches any
    pub like: Option<String>,
    /// Strictly equal to
    pub equal: Option<String>,
    /// Starts with
    pub start: Option<String>,
    /// Ends with
    pub end: Option<String>,
    /// Matches the regex
    pub regex: Option<String>,
}

/// The response recieved from a query
/// Contains all the parameters that were initially passed in (if they were lost)
/// Contains the number of pages returned, their sizes and the items for current page
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PaginatedResponseWeb<T> {
    /// The vector query
    pub vec_query: Option<VecQueryWeb>,
    /// The ordering
    pub order_query: Option<OrderQueryWeb>,
    /// The string query
    pub string_query: Option<StringQueryWeb>,
    /// The range query
    pub range_query: Option<RangeQueryWeb>,
    /// Total number of items found
    pub num_items: u64,
    /// Total number of pages available
    pub num_pages: u64,
    /// Size of each page
    pub page_size: u64,
    /// The current page number
    pub current_page_num: u64,
    /// The current items in the page
    pub current_page_items: Vec<T>,
}

/// The query submitted to the API
/// Traits locked behind the `openapi` feature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct FullQueryWeb {
    /// Page settings
    pub page_query: PageQuery,
    /// Query for vectors
    pub vec_query: Option<VecQueryWeb>,
    /// Ordering of items
    pub order_query: Option<OrderQueryWeb>,
    /// Query for strings
    pub string_query: Option<StringQueryWeb>,
    /// Query for range
    pub range_query: Option<RangeQueryWeb>,
}

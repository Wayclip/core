use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum Order {
    ASC,
    DESC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PageQuery {
    pub page_size: u64,
    pub page_num: u64,
}

// external types for routes, etc... (basically made so they have no generics and we can convert
// properlly)

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VecQueryWeb {
    pub column: String,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub any_of: Option<Vec<serde_json::Value>>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub none_of: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct OrderQueryWeb {
    pub column: String,
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RangeQueryWeb {
    pub column: String,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub greater_than: Option<serde_json::Value>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub greater_than_or_equal: Option<serde_json::Value>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub less_than: Option<serde_json::Value>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Vec<Object>>))]
    pub less_than_or_equal: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct StringQueryWeb {
    pub column: String,
    pub like: Option<String>,
    pub equal: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PaginatedResponseWeb<T> {
    pub vec_query: Option<VecQueryWeb>,
    pub order_query: Option<OrderQueryWeb>,
    pub string_query: Option<StringQueryWeb>,
    pub range_query: Option<RangeQueryWeb>,
    pub num_items: u64,
    pub num_pages: u64,
    pub page_size: u64,
    pub current_page_num: u64,
    pub current_page_items: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct FullQueryWeb {
    pub page_query: PageQuery,
    pub vec_query: Option<VecQueryWeb>,
    pub order_query: Option<OrderQueryWeb>,
    pub string_query: Option<StringQueryWeb>,
    pub range_query: Option<RangeQueryWeb>,
}

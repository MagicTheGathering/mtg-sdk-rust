use reqwest::header::HeaderMap;

/// Response returned by the Cards API
#[allow(dead_code)]
pub struct ApiResponse<T> {
    pub content: T,
    pub page_size: Option<u32>,
    pub count: Option<u32>,
    pub total_count: Option<u32>,
    pub ratelimit_limit: Option<u32>,
    pub ratelimit_remaining: Option<u32>,
}

impl<T> ApiResponse<T> {
    pub(crate) fn new(content: T, headers: HeaderMap) -> ApiResponse<T> {
        let page_size: Option<u32> = if headers.contains_key("Page-Size") { Some(headers.get("Page-Size").unwrap().to_str().expect("Error parsing Page-Size Header").parse::<u32>().expect("Error converting str to u32")) } else { None };
        let count: Option<u32> = if headers.contains_key("Count") { Some(headers.get("Count").unwrap().to_str().expect("Error parsing Count Header").parse::<u32>().expect("Error converting str to u32")) } else { None };
        let total_count: Option<u32> = if headers.contains_key("Total-Count") { Some(headers.get("Total-Count").unwrap().to_str().expect("Error parsing Total-Count Header").parse::<u32>().expect("Error converting str to u32")) } else { None };
        let ratelimit_limit: Option<u32> = if headers.contains_key("Ratelimit-Limit") { Some(headers.get("Ratelimit-Limit").unwrap().to_str().expect("Error parsing Ratelimit-Limit Header").parse::<u32>().expect("Error converting str to u32")) } else { None };
        let ratelimit_remaining = if headers.contains_key("Ratelimit-Remaining") { Some(headers.get("Ratelimit-Remaining").unwrap().to_str().expect("Error parsing Ratelimit-Remaining Header").parse::<u32>().expect("Error converting str to u32")) } else { None };
        ApiResponse {
            content,
            page_size,
            count,
            total_count,
            ratelimit_limit,
            ratelimit_remaining,
        }
    }
}

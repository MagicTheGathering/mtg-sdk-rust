use hyper::Headers;
header! { (PageSizeHeader, "Page-Size") => [u32] }
header! { (CountHeader, "Count") => [u32] }
header! { (TotalCountHeader, "Total-Count") => [u32] }
header! { (RatelimitLimitHeader, "Ratelimit-Limit") => [u32] }
header! { (RatelimitRemainingHeader, "Ratelimit-Remaining") => [u32] }

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
    pub(crate) fn new(content: T, headers: &Headers) -> ApiResponse<T> {
        let page_size = headers.get::<PageSizeHeader>().map(|header| header.0);
        let count = headers.get::<CountHeader>().map(|header| header.0);
        let total_count = headers.get::<TotalCountHeader>().map(|header| header.0);
        let ratelimit_limit = headers.get::<RatelimitLimitHeader>().map(|header| header.0);
        let ratelimit_remaining = headers
            .get::<RatelimitRemainingHeader>()
            .map(|header| header.0);
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

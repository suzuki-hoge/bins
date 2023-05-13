use crate::item::http::fetch_pull_requests as fetch;
use crate::item::pull_request_item::PullRequestItem;

mod http;
mod pull_request_item;

pub async fn fetch_pull_requests() -> Vec<PullRequestItem> {
    match fetch().await {
        Ok(items) => items,
        Err(_) => vec![],
    }
}

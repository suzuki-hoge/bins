extern crate bins;

use std::cmp::max;
use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde::Deserialize;

use bins::git::config::{get_current_git_config, GitConfig};
use bins::git::token::get_git_token;

use crate::item::pull_request_item::PullRequestItem;

pub async fn fetch_pull_requests() -> anyhow::Result<Vec<PullRequestItem>> {
    let git_config = match get_current_git_config() {
        Some(git_config) => Ok(git_config),
        None => Err(anyhow!("not git directory")),
    }?;
    let token = get_git_token();

    let client = Client::new();
    let url = "https://api.github.com/graphql";
    let data = vec![("query", get_query(&git_config))].into_iter().collect::<HashMap<_, _>>();

    let response = client.post(url).bearer_auth(token).header(USER_AGENT, "curl").json(&data).send().await?;
    let body = response.text().await?;
    let data: Data = serde_json::from_str(&body)?;

    let mut max_number = 0;
    let mut max_author_length = 0;
    let mut max_branch_length = 0;

    for pull_request_node in data.data.repository.pull_requests.nodes.iter() {
        max_number = max(max_number, pull_request_node.number);
        max_author_length = max(max_author_length, pull_request_node.author.login.len());
        max_branch_length = max(max_branch_length, pull_request_node.head_ref_name.len());
    }

    Ok(data
        .data
        .repository
        .pull_requests
        .nodes
        .iter()
        .map(|pull_request_node| {
            PullRequestItem::new(
                &git_config,
                (pull_request_node.number, max_number),
                (pull_request_node.author.login.to_string(), max_author_length),
                (pull_request_node.head_ref_name.to_string(), max_branch_length),
                pull_request_node.title.to_string(),
                pull_request_node
                    .review_requests
                    .nodes
                    .iter()
                    .map(|review_request_node| review_request_node.requested_reviewer.user.to_string())
                    .unique()
                    .collect(),
                pull_request_node
                    .reviews
                    .nodes
                    .iter()
                    .map(|review_node| review_node.author.login.to_string())
                    .unique()
                    .collect(),
            )
        })
        .collect())
}

fn get_query(git_config: &GitConfig) -> String {
    "query {
      repository(owner: \"{owner}\", name: \"{repo}\") {
        pullRequests(last: 50, states: OPEN) {
          nodes {
            number
            author {
              login
            }
            headRefName
            title
            reviewRequests(first: 10) {
              nodes {
                requestedReviewer {
                  ... on User {
                    user: login
                  }
                }
              }
            }
            reviews(first: 50) {
              nodes {
                author {
                  login
                }
              }
            }
          }
        }
      }
    }"
    .replace("{owner}", &git_config.owner)
    .replace("{repo}", &git_config.repo)
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Repository,
}

#[derive(Deserialize, Debug)]
struct Repository {
    repository: PullRequests,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PullRequests {
    pull_requests: PullRequestNodes,
}

#[derive(Deserialize, Debug)]
struct PullRequestNodes {
    nodes: Vec<PullRequestNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PullRequestNode {
    title: String,
    number: u64,
    head_ref_name: String,
    author: Author,
    review_requests: ReviewRequestNodes,
    reviews: ReviewNodes,
}

#[derive(Deserialize, Debug)]
struct ReviewRequestNodes {
    nodes: Vec<ReviewRequestNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReviewRequestNode {
    requested_reviewer: RequestedReviewer,
}

#[derive(Deserialize, Debug)]
struct RequestedReviewer {
    user: String,
}

#[derive(Deserialize, Debug)]
struct ReviewNodes {
    nodes: Vec<ReviewNode>,
}

#[derive(Deserialize, Debug)]
struct ReviewNode {
    author: Author,
}

#[derive(Deserialize, Debug)]
struct Author {
    login: String,
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub enum ReviewOutcome {
    #[serde(rename = "pull_request_review_approved")]
    Approved,
    #[serde(rename = "pull_request_review_rejected")]
    Rejected,
    #[serde(rename = "pull_request_review_comment")]
    Comment,
}

#[derive(Deserialize, Debug)]
pub struct Review {
    pub content: String,
    pub r#type: ReviewOutcome,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub body: String,
    pub comments: u64,
    pub id: u64,
    pub user: User,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum Action {
    Opened,
    Closed,
    Reopened,
    Edited,
    Reviewed { review: Review },
    ReviewRequested { requested_reviewer: User },
}

#[derive(Deserialize, Debug)]
pub struct Webhook {
    #[serde(flatten)]
    pub action: Action,
    pub pull_request: PullRequest,
    pub sender: User,
}

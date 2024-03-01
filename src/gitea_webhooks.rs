use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Review {
    pub content: String,
    pub r#type: String,
}

// enum Review {
//     Approved { comment: String },
//     Rejected { comment: String },
//     Comment { comment: String },
// }

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
#[serde(rename_all = "snake_case")]
pub enum Action {
    Opened,
    Closed,
    Reopened,
    Edited,
    Reviewed,
    ReviewRequested,
}

#[derive(Deserialize, Debug)]
pub struct Webhook {
    pub action: Action,
    pub pull_request: PullRequest,
    pub requested_reviewer: Option<User>,
    pub review: Option<Review>,
    pub sender: User,
}

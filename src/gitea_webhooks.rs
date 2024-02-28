use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Review {
    pub content: String,
    pub r#type: String,
}

// enum Review {
//     Approved { comment: String },
//     Rejected { comment: String },
//     Comment { comment: String },
// }

#[derive(Deserialize)]
pub struct PullRequest {
    pub body: String,
    pub comments: u64,
    pub id: u64,
    pub user: User,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Webhook {
    pub action: String,
    pub pull_request: PullRequest,
    pub requested_reviewer: Option<User>,
    pub review: Option<Review>,
    pub sender: User,
}

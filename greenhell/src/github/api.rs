//! GitHub API request types.

use super::{Branch, Client, PullRequestNumber, Ref, Repository, Sha};

/// Derives `Clone`, `Debug`, `PartialEq`, and `serde::Deserialize`.
///
/// Note: This macro intentionally does not use `#[serde(deny_unknown_fields)]`.
/// GitHub's API versioning policy explicitly allows adding new response fields
/// as non-breaking changes, even for pinned API versions:
/// <https://docs.github.com/en/rest/about-the-rest-api/api-versions>
///
/// Therefore, our types only include fields we actually use, and unknown fields
/// are silently ignored to maintain compatibility with future API additions.
macro_rules! serde_derive {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        $vis struct $name {
            $(
                $(#[$field_attr])*
                $field_vis $field : $ty
            ),*
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "snake_case")]
        $vis enum $name {
            $($variant),*
        }
    };
}

serde_derive! {
    /// Response from `GET /repos/{owner}/{repo}`.
    pub struct RepositoryInfo {
        pub id: u64,
        pub name: String,
        pub full_name: String,
        pub default_branch: String,
    }
}

/// Get repository information.
///
/// `GET /repos/{owner}/{repo}`
pub struct GetRepository {
    pub repository: Repository,
}

impl mhttp::Request<Client> for GetRepository {
    type Response = RepositoryInfo;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client.get(base_url.set_path_segments(&[
            "repos",
            self.repository.owner(),
            self.repository.repo(),
        ]))
    }
}

serde_derive! {
    /// Response from `GET /repos/{owner}/{repo}/compare/{base}...{head}`.
    pub struct Comparison {
        pub status: ComparisonStatus,
        pub ahead_by: u64,
        pub behind_by: u64,
        pub commits: Vec<Commit>,
    }
}

serde_derive! {
    /// Status of a branch comparison.
    pub enum ComparisonStatus {
        Ahead,
        Behind,
        Diverged,
        Identical,
    }
}

serde_derive! {
    /// A commit from the GitHub API.
    pub struct Commit {
        pub sha: Sha,
    }
}

/// Compare two commits.
///
/// `GET /repos/{owner}/{repo}/compare/{base}...{head}`
pub struct CompareCommits {
    pub repository: Repository,
    pub base: Ref,
    pub head: Ref,
}

impl mhttp::Request<Client> for CompareCommits {
    type Response = mhttp::link::Paginated<Comparison>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        let compare_ref = format!("{}...{}", self.base, self.head);
        client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "compare",
                &compare_ref,
            ]))
            .query(&[("per_page", MAX_PER_PAGE)])
    }
}

impl mhttp::link::PaginatedRequest for CompareCommits {}

serde_derive! {
    /// Response from `GET /repos/{owner}/{repo}/commits/{ref}/check-runs`.
    pub struct CheckRunList {
        pub total_count: u64,
        pub check_runs: Vec<CheckRun>,
    }
}

/// Name of a check run.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct CheckName(String);

impl CheckName {
    /// Creates a new check name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Returns the check name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if this is the greenhell check.
    #[must_use]
    pub fn is_greenhell(&self) -> bool {
        self == &*crate::evaluate::GREENHELL
    }
}

impl std::fmt::Display for CheckName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<&str> for CheckName {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for CheckName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

serde_derive! {
    /// A check run from the GitHub API.
    pub struct CheckRun {
        pub id: u64,
        pub name: CheckName,
        pub status: CheckRunStatus,
        pub conclusion: Option<CheckRunConclusion>,
    }
}

serde_derive! {
    /// Status of a check run.
    pub enum CheckRunStatus {
        Queued,
        InProgress,
        Completed,
    }
}

serde_derive! {
    /// Conclusion of a completed check run.
    pub enum CheckRunConclusion {
        ActionRequired,
        Cancelled,
        Failure,
        Neutral,
        Skipped,
        Stale,
        Success,
        TimedOut,
    }
}

/// Maximum items per page for GitHub API pagination.
pub const MAX_PER_PAGE: u8 = 100;

/// List check runs for a commit.
///
/// `GET /repos/{owner}/{repo}/commits/{ref}/check-runs`
pub struct ListCheckRuns {
    pub repository: Repository,
    pub git_ref: Ref,
}

impl ListCheckRuns {
    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "commits",
                self.git_ref.as_str(),
                "check-runs",
            ]))
            .query(&[("per_page", MAX_PER_PAGE)])
    }
}

impl mhttp::Request<Client> for ListCheckRuns {
    type Response = mhttp::link::Paginated<CheckRunList>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        Self::request_builder(self, client, base_url)
    }
}

impl mhttp::link::PaginatedRequest for ListCheckRuns {}

/// `GET /repos/{owner}/{repo}/commits/{ref}/check-runs`
///
/// Returns `None` if the commit doesn't exist on GitHub (422 status).
pub struct TryListCheckRuns(pub ListCheckRuns);

impl mhttp::Request<Client> for TryListCheckRuns {
    type Response = Option<CheckRunList>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json_map(http::StatusCode::OK, Some)
            .status_code(http::StatusCode::UNPROCESSABLE_ENTITY, |content_types| {
                content_types.constant(None);
            })
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        self.0.request_builder(client, base_url)
    }
}

serde_derive! {
    /// Combined status state.
    pub enum CombinedStatusState {
        Failure,
        Pending,
        Success,
    }
}

serde_derive! {
    /// Individual commit status.
    pub struct CommitStatus {
        pub id: u64,
        pub state: CommitStatusState,
        pub context: CheckName,
        pub description: Option<String>,
        pub target_url: Option<String>,
    }
}

serde_derive! {
    /// Response from `GET /repos/{owner}/{repo}/commits/{ref}/status`.
    pub struct CombinedStatus {
        pub state: CombinedStatusState,
        pub total_count: u64,
        pub statuses: Vec<CommitStatus>,
        pub sha: Sha,
    }
}

/// Get combined status for a commit.
///
/// `GET /repos/{owner}/{repo}/commits/{ref}/status`
pub struct GetCombinedStatus {
    pub repository: Repository,
    pub git_ref: Ref,
}

impl mhttp::Request<Client> for GetCombinedStatus {
    type Response = CombinedStatus;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client.get(base_url.set_path_segments(&[
            "repos",
            self.repository.owner(),
            self.repository.repo(),
            "commits",
            self.git_ref.as_str(),
            "status",
        ]))
    }
}

serde_derive! {
    /// Response from check run create/update.
    pub struct CheckRunResponse {
        pub id: u64,
        pub name: CheckName,
        pub status: CheckRunStatus,
        pub conclusion: Option<CheckRunConclusion>,
    }
}

/// Create a check run.
///
/// `POST /repos/{owner}/{repo}/check-runs`
#[derive(Debug)]
pub struct CreateCheckRun {
    pub repository: Repository,
    pub name: CheckName,
    pub head_sha: Sha,
    pub status: CheckRunStatus,
    pub conclusion: Option<CheckRunConclusion>,
    pub output: Option<CheckRunOutput>,
}

/// Check run output for detailed information.
#[derive(Debug, PartialEq, serde::Serialize)]
pub struct CheckRunOutput {
    pub title: String,
    pub summary: String,
}

impl mhttp::Request<Client> for CreateCheckRun {
    type Response = CheckRunResponse;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::CREATED)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            name: &'a CheckName,
            head_sha: &'a Sha,
            status: &'a CheckRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            conclusion: Option<&'a CheckRunConclusion>,
            #[serde(skip_serializing_if = "Option::is_none")]
            output: Option<&'a CheckRunOutput>,
        }

        client
            .post(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "check-runs",
            ]))
            .json(&Body {
                name: &self.name,
                head_sha: &self.head_sha,
                status: &self.status,
                conclusion: self.conclusion.as_ref(),
                output: self.output.as_ref(),
            })
    }
}

/// Update a check run.
///
/// `PATCH /repos/{owner}/{repo}/check-runs/{check_run_id}`
pub struct UpdateCheckRun {
    pub repository: Repository,
    pub check_run_id: u64,
    pub status: Option<CheckRunStatus>,
    pub conclusion: Option<CheckRunConclusion>,
    pub output: Option<CheckRunOutput>,
}

impl mhttp::Request<Client> for UpdateCheckRun {
    type Response = CheckRunResponse;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<&'a CheckRunStatus>,
            #[serde(skip_serializing_if = "Option::is_none")]
            conclusion: Option<&'a CheckRunConclusion>,
            #[serde(skip_serializing_if = "Option::is_none")]
            output: Option<&'a CheckRunOutput>,
        }

        client
            .patch(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "check-runs",
                &self.check_run_id.to_string(),
            ]))
            .json(&Body {
                status: self.status.as_ref(),
                conclusion: self.conclusion.as_ref(),
                output: self.output.as_ref(),
            })
    }
}

/// State for a commit status.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitStatusState {
    /// The status is pending.
    Pending,
    /// The status is successful.
    Success,
    /// The status has failed.
    Failure,
    /// The status has errored.
    Error,
}

/// Create a commit status.
///
/// `POST /repos/{owner}/{repo}/statuses/{sha}`
#[derive(Debug)]
pub struct CreateCommitStatus {
    pub repository: Repository,
    pub sha: Sha,
    pub state: CommitStatusState,
    pub context: CheckName,
    pub description: Option<String>,
}

impl mhttp::Request<Client> for CreateCommitStatus {
    type Response = CommitStatus;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::CREATED)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            state: &'a CommitStatusState,
            context: &'a CheckName,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<&'a str>,
        }

        client
            .post(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "statuses",
                self.sha.as_str(),
            ]))
            .json(&Body {
                state: &self.state,
                context: &self.context,
                description: self.description.as_deref(),
            })
    }
}

serde_derive! {
    /// Response from `GET /repos/{owner}/{repo}/pulls/{pull_number}`.
    pub struct PullRequestInfo {
        pub id: u64,
        pub url: url::Url,
        pub html_url: url::Url,
        pub diff_url: url::Url,
        pub patch_url: url::Url,
        pub issue_url: url::Url,
        pub commits_url: url::Url,
        pub review_comments_url: url::Url,
        pub review_comment_url: url::Url,
        pub comments_url: url::Url,
        pub statuses_url: url::Url,
        pub number: u64,
        pub state: PullRequestState,
        pub locked: bool,
        pub title: String,
        pub user: serde_json::Value,
        pub body: Option<String>,
        pub labels: serde_json::Value,
        pub milestone: Option<serde_json::Value>,
        pub active_lock_reason: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub closed_at: Option<String>,
        pub merged_at: Option<String>,
        pub merge_commit_sha: Option<String>,
        pub assignee: Option<serde_json::Value>,
        pub assignees: serde_json::Value,
        pub requested_reviewers: serde_json::Value,
        pub requested_teams: serde_json::Value,
        pub head: PullRequestRef,
        pub base: PullRequestRef,
        #[serde(rename = "_links")]
        pub links: serde_json::Value,
        pub author_association: String,
        pub auto_merge: Option<serde_json::Value>,
        pub draft: bool,
    }
}

serde_derive! {
    /// State of a pull request.
    pub enum PullRequestState {
        Open,
        Closed,
    }
}

serde_derive! {
    /// A ref in a pull request (head or base).
    pub struct PullRequestRef {
        pub label: String,
        pub r#ref: String,
        pub sha: Sha,
        pub user: serde_json::Value,
        pub repo: serde_json::Value,
    }
}

/// Get pull request information.
///
/// `GET /repos/{owner}/{repo}/pulls/{pull_number}`
pub struct GetPullRequest {
    pub repository: Repository,
    pub pull_request: PullRequestNumber,
}

impl mhttp::Request<Client> for GetPullRequest {
    type Response = PullRequestInfo;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client.get(base_url.set_path_segments(&[
            "repos",
            self.repository.owner(),
            self.repository.repo(),
            "pulls",
            &self.pull_request.to_string(),
        ]))
    }
}

/// List pull requests.
///
/// `GET /repos/{owner}/{repo}/pulls`
pub struct ListPullRequests {
    pub repository: Repository,
}

impl mhttp::Request<Client> for ListPullRequests {
    type Response = mhttp::link::Paginated<Vec<PullRequestInfo>>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "pulls",
            ]))
            .query(&[("per_page", MAX_PER_PAGE)])
    }
}

impl mhttp::link::PaginatedRequest for ListPullRequests {}

/// List pull requests by head branch.
///
/// `GET /repos/{owner}/{repo}/pulls?head={owner}:{branch}`
pub struct ListPullRequestsByHead {
    pub repository: Repository,
    pub branch: Branch,
}

impl mhttp::Request<Client> for ListPullRequestsByHead {
    type Response = mhttp::link::Paginated<Vec<PullRequestInfo>>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        let head = format!("{}:{}", self.repository.owner(), self.branch);
        client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "pulls",
            ]))
            .query(&[("head", head), ("per_page", MAX_PER_PAGE.to_string())])
    }
}

impl mhttp::link::PaginatedRequest for ListPullRequestsByHead {}

serde_derive! {
    /// A commit from `GET /repos/{owner}/{repo}/pulls/{pull_number}/commits`.
    pub struct PullRequestCommit {
        pub url: url::Url,
        pub sha: Sha,
        pub node_id: String,
        pub html_url: url::Url,
        pub comments_url: url::Url,
        pub commit: CommitDetails,
        pub author: Option<serde_json::Value>,
        pub committer: Option<serde_json::Value>,
        pub parents: Vec<CommitParent>,
    }
}

serde_derive! {
    /// Commit details nested in a commit object.
    pub struct CommitDetails {
        pub url: url::Url,
        pub author: GitUser,
        pub committer: GitUser,
        pub message: String,
        pub comment_count: u64,
        pub tree: CommitTree,
        pub verification: CommitVerification,
    }
}

serde_derive! {
    /// Git user info (author/committer).
    pub struct GitUser {
        pub name: String,
        pub email: String,
        pub date: String,
    }
}

serde_derive! {
    /// Tree reference in a commit.
    pub struct CommitTree {
        pub url: url::Url,
        pub sha: Sha,
    }
}

serde_derive! {
    /// Commit signature verification.
    pub struct CommitVerification {
        pub verified: bool,
        pub reason: String,
        pub signature: Option<String>,
        pub payload: Option<String>,
    }
}

serde_derive! {
    /// Parent commit reference.
    pub struct CommitParent {
        pub url: url::Url,
        pub sha: Sha,
        pub html_url: url::Url,
    }
}

/// List commits on a pull request.
///
/// `GET /repos/{owner}/{repo}/pulls/{pull_number}/commits`
pub struct ListPullRequestCommits {
    pub repository: Repository,
    pub pull_request: PullRequestNumber,
}

impl mhttp::Request<Client> for ListPullRequestCommits {
    type Response = mhttp::link::Paginated<Vec<PullRequestCommit>>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "pulls",
                &self.pull_request.to_string(),
                "commits",
            ]))
            .query(&[("per_page", MAX_PER_PAGE)])
    }
}

impl mhttp::link::PaginatedRequest for ListPullRequestCommits {}

serde_derive! {
    /// Event from GitHub Events API.
    pub struct Event {
        pub id: String,
        #[serde(rename = "type")]
        pub event_type: String,
        pub created_at: String,
        #[serde(default)]
        pub payload: serde_json::Value,
    }
}

/// Events page response from GitHub Events API.
#[derive(Clone, Debug, PartialEq)]
pub enum EventsPage {
    /// Content with events and headers.
    Content {
        /// The events in this page.
        events: Vec<Event>,
        /// Server-recommended polling interval in seconds (from X-Poll-Interval header).
        poll_interval: u64,
        /// ETag for conditional requests (from ETag header).
        etag: String,
    },
    /// Not modified (304) - no new events since last request.
    NotModified,
}

/// Headers extracted from events API response.
struct EventsHeaders {
    poll_interval: u64,
    etag: String,
}

fn extract_events_headers(
    headers: &http::HeaderMap,
) -> Result<EventsHeaders, mhttp::decoder::DecodeError> {
    const X_POLL_INTERVAL: http::header::HeaderName =
        http::header::HeaderName::from_static("x-poll-interval");

    // GitHub's default 60s poll interval is too slow for responsive CLI feedback.
    // We override it to 5s for better UX while still respecting rate limits.
    let poll_interval = match headers.get(&X_POLL_INTERVAL) {
        None => {
            return Err(mhttp::decoder::DecodeError {
                reason: mhttp::decoder::ErrorReason::MissingHeader {
                    name: X_POLL_INTERVAL,
                },
                source: None,
            });
        }
        Some(value) => {
            let value: u64 = value
                .to_str()
                .ok()
                .and_then(|value| value.parse().ok())
                .ok_or_else(|| mhttp::decoder::DecodeError {
                    reason: mhttp::decoder::ErrorReason::InvalidHeaderValue {
                        name: X_POLL_INTERVAL,
                    },
                    source: None,
                })?;
            if value == 60 { 5 } else { value }
        }
    };

    let etag = headers
        .get(http::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string())
        .unwrap_or_default();

    Ok(EventsHeaders {
        poll_interval,
        etag,
    })
}

fn decode_events_page(
    headers: EventsHeaders,
    body: &[u8],
) -> Result<EventsPage, mhttp::decoder::DecodeError> {
    let events: Vec<Event> = mhttp::decoder::json(body)?;
    Ok(EventsPage::Content {
        events,
        poll_interval: headers.poll_interval,
        etag: headers.etag,
    })
}

/// List repository events.
///
/// `GET /repos/{owner}/{repo}/events`
///
/// Returns up to 300 events from the last 30 days.
/// If `etag` is provided, sends `If-None-Match` header for conditional request.
pub struct ListRepositoryEvents {
    pub repository: Repository,
    /// ETag from previous request for conditional polling.
    pub etag: Option<String>,
}

impl mhttp::Request<Client> for ListRepositoryEvents {
    type Response = mhttp::link::Paginated<EventsPage>;

    mhttp::decoder!(
        mhttp::decoder::Response::build()
            .status_code(http::StatusCode::OK, |content_types| {
                content_types.add_with_headers(
                    "application/json",
                    extract_events_headers,
                    decode_events_page,
                );
                content_types.add_with_headers(
                    "application/json; charset=utf-8",
                    extract_events_headers,
                    decode_events_page,
                );
            })
            .status_code(http::StatusCode::NOT_MODIFIED, |content_types| {
                content_types.constant(EventsPage::NotModified);
            })
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &mhttp::BaseUrl,
    ) -> reqwest::RequestBuilder {
        let builder = client
            .get(base_url.set_path_segments(&[
                "repos",
                self.repository.owner(),
                self.repository.repo(),
                "events",
            ]))
            .query(&[("per_page", MAX_PER_PAGE)]);

        match &self.etag {
            Some(etag) => builder.header(http::header::IF_NONE_MATCH, etag),
            None => builder,
        }
    }
}

impl mhttp::link::PaginatedRequest for ListRepositoryEvents {}

#[cfg(test)]
mod tests {
    use super::*;
    use mhttp::Request;
    use mhttp::testing::TestRequest;

    fn base_url() -> mhttp::BaseUrl {
        mhttp::BaseUrl::https(url::Host::parse("api.github.com").unwrap())
    }

    fn json_response(status: http::StatusCode, body: &'static str) -> reqwest::Response {
        http::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap()
            .into()
    }

    mod get_repository {
        use super::*;

        #[test]
        fn request_builder() {
            let request = GetRepository {
                repository: "mbj/mrs".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs".parse().unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"{
                    "id": 123456,
                    "name": "mrs",
                    "full_name": "mbj/mrs",
                    "default_branch": "main"
                }"#,
            );

            let result = GetRepository::DECODER.decode(response).await.unwrap();

            assert_eq!(
                result,
                RepositoryInfo {
                    id: 123456,
                    name: "mrs".to_string(),
                    full_name: "mbj/mrs".to_string(),
                    default_branch: "main".to_string(),
                }
            );
        }
    }

    mod compare_commits {
        use super::*;

        #[test]
        fn request_builder() {
            let request = CompareCommits {
                repository: "mbj/mrs".parse().unwrap(),
                base: "main".parse().unwrap(),
                head: "feature".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/compare/main...feature?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[test]
        fn request_builder_encodes_refs() {
            let request = CompareCommits {
                repository: "mbj/mrs".parse().unwrap(),
                base: "main".parse().unwrap(),
                head: "feature/test".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/compare/main...feature%2Ftest?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"{
                    "status": "ahead",
                    "ahead_by": 2,
                    "behind_by": 0,
                    "commits": [
                        {"sha": "abc123abc123abc123abc123abc123abc123abc1"},
                        {"sha": "def456def456def456def456def456def456def4"}
                    ]
                }"#,
            );

            let result = CompareCommits::DECODER.decode(response).await.unwrap();

            assert_eq!(
                result,
                mhttp::link::Paginated {
                    data: Comparison {
                        status: ComparisonStatus::Ahead,
                        ahead_by: 2,
                        behind_by: 0,
                        commits: vec![
                            Commit {
                                sha: "abc123abc123abc123abc123abc123abc123abc1".parse().unwrap()
                            },
                            Commit {
                                sha: "def456def456def456def456def456def456def4".parse().unwrap()
                            },
                        ],
                    },
                    links: None,
                }
            );
        }
    }

    mod list_check_runs {
        use super::*;

        #[test]
        fn request_builder() {
            let request = ListCheckRuns {
                repository: "mbj/mrs".parse().unwrap(),
                git_ref: "abc123".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/commits/abc123/check-runs?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[test]
        fn request_builder_encodes_ref() {
            let request = ListCheckRuns {
                repository: "mbj/mrs".parse().unwrap(),
                git_ref: "feature/test".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/commits/feature%2Ftest/check-runs?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"{
                    "total_count": 2,
                    "check_runs": [
                        {
                            "id": 1,
                            "name": "build",
                            "status": "completed",
                            "conclusion": "success"
                        },
                        {
                            "id": 2,
                            "name": "test",
                            "status": "in_progress",
                            "conclusion": null
                        }
                    ]
                }"#,
            );

            let result = ListCheckRuns::DECODER.decode(response).await.unwrap();

            assert_eq!(
                result,
                mhttp::link::Paginated {
                    data: CheckRunList {
                        total_count: 2,
                        check_runs: vec![
                            CheckRun {
                                id: 1,
                                name: "build".into(),
                                status: CheckRunStatus::Completed,
                                conclusion: Some(CheckRunConclusion::Success),
                            },
                            CheckRun {
                                id: 2,
                                name: "test".into(),
                                status: CheckRunStatus::InProgress,
                                conclusion: None,
                            },
                        ],
                    },
                    links: None,
                }
            );
        }
    }

    mod get_pull_request {
        use super::*;

        #[test]
        fn request_builder() {
            let request = GetPullRequest {
                repository: "mbj/mrs".parse().unwrap(),
                pull_request: "123".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/pulls/123"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"{
                    "id": 1,
                    "url": "https://api.github.com/repos/mbj/mrs/pulls/123",
                    "html_url": "https://github.com/mbj/mrs/pull/123",
                    "diff_url": "https://github.com/mbj/mrs/pull/123.diff",
                    "patch_url": "https://github.com/mbj/mrs/pull/123.patch",
                    "issue_url": "https://api.github.com/repos/mbj/mrs/issues/123",
                    "commits_url": "https://api.github.com/repos/mbj/mrs/pulls/123/commits",
                    "review_comments_url": "https://api.github.com/repos/mbj/mrs/pulls/123/comments",
                    "review_comment_url": "https://api.github.com/repos/mbj/mrs/pulls/comments{/number}",
                    "comments_url": "https://api.github.com/repos/mbj/mrs/issues/123/comments",
                    "statuses_url": "https://api.github.com/repos/mbj/mrs/statuses/abc123",
                    "number": 123,
                    "state": "open",
                    "locked": false,
                    "title": "Test PR",
                    "user": {"login": "mbj"},
                    "body": "PR body",
                    "labels": [],
                    "milestone": null,
                    "active_lock_reason": null,
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z",
                    "closed_at": null,
                    "merged_at": null,
                    "merge_commit_sha": null,
                    "assignee": null,
                    "assignees": [],
                    "requested_reviewers": [],
                    "requested_teams": [],
                    "head": {
                        "label": "mbj:feature",
                        "ref": "feature",
                        "sha": "abc123abc123abc123abc123abc123abc123abc1",
                        "user": {"login": "mbj"},
                        "repo": {"name": "mrs"}
                    },
                    "base": {
                        "label": "mbj:main",
                        "ref": "main",
                        "sha": "def456def456def456def456def456def456def4",
                        "user": {"login": "mbj"},
                        "repo": {"name": "mrs"}
                    },
                    "_links": {},
                    "author_association": "OWNER",
                    "auto_merge": null,
                    "draft": false
                }"#,
            );

            let result = GetPullRequest::DECODER.decode(response).await.unwrap();

            assert_eq!(
                result,
                PullRequestInfo {
                    id: 1,
                    url: "https://api.github.com/repos/mbj/mrs/pulls/123"
                        .parse()
                        .unwrap(),
                    html_url: "https://github.com/mbj/mrs/pull/123".parse().unwrap(),
                    diff_url: "https://github.com/mbj/mrs/pull/123.diff".parse().unwrap(),
                    patch_url: "https://github.com/mbj/mrs/pull/123.patch".parse().unwrap(),
                    issue_url: "https://api.github.com/repos/mbj/mrs/issues/123"
                        .parse()
                        .unwrap(),
                    commits_url: "https://api.github.com/repos/mbj/mrs/pulls/123/commits"
                        .parse()
                        .unwrap(),
                    review_comments_url: "https://api.github.com/repos/mbj/mrs/pulls/123/comments"
                        .parse()
                        .unwrap(),
                    review_comment_url:
                        "https://api.github.com/repos/mbj/mrs/pulls/comments{/number}"
                            .parse()
                            .unwrap(),
                    comments_url: "https://api.github.com/repos/mbj/mrs/issues/123/comments"
                        .parse()
                        .unwrap(),
                    statuses_url: "https://api.github.com/repos/mbj/mrs/statuses/abc123"
                        .parse()
                        .unwrap(),
                    number: 123,
                    state: PullRequestState::Open,
                    locked: false,
                    title: "Test PR".to_string(),
                    user: serde_json::json!({"login": "mbj"}),
                    body: Some("PR body".to_string()),
                    labels: serde_json::json!([]),
                    milestone: None,
                    active_lock_reason: None,
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    updated_at: "2024-01-01T00:00:00Z".to_string(),
                    closed_at: None,
                    merged_at: None,
                    merge_commit_sha: None,
                    assignee: None,
                    assignees: serde_json::json!([]),
                    requested_reviewers: serde_json::json!([]),
                    requested_teams: serde_json::json!([]),
                    head: PullRequestRef {
                        label: "mbj:feature".to_string(),
                        r#ref: "feature".to_string(),
                        sha: "abc123abc123abc123abc123abc123abc123abc1".parse().unwrap(),
                        user: serde_json::json!({"login": "mbj"}),
                        repo: serde_json::json!({"name": "mrs"}),
                    },
                    base: PullRequestRef {
                        label: "mbj:main".to_string(),
                        r#ref: "main".to_string(),
                        sha: "def456def456def456def456def456def456def4".parse().unwrap(),
                        user: serde_json::json!({"login": "mbj"}),
                        repo: serde_json::json!({"name": "mrs"}),
                    },
                    links: serde_json::json!({}),
                    author_association: "OWNER".to_string(),
                    auto_merge: None,
                    draft: false,
                }
            );
        }
    }

    mod list_pull_requests {
        use super::*;

        #[test]
        fn request_builder() {
            let request = ListPullRequests {
                repository: "mbj/mrs".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/pulls?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"[
                    {
                        "id": 1,
                        "url": "https://api.github.com/repos/mbj/mrs/pulls/123",
                        "html_url": "https://github.com/mbj/mrs/pull/123",
                        "diff_url": "https://github.com/mbj/mrs/pull/123.diff",
                        "patch_url": "https://github.com/mbj/mrs/pull/123.patch",
                        "issue_url": "https://api.github.com/repos/mbj/mrs/issues/123",
                        "commits_url": "https://api.github.com/repos/mbj/mrs/pulls/123/commits",
                        "review_comments_url": "https://api.github.com/repos/mbj/mrs/pulls/123/comments",
                        "review_comment_url": "https://api.github.com/repos/mbj/mrs/pulls/comments{/number}",
                        "comments_url": "https://api.github.com/repos/mbj/mrs/issues/123/comments",
                        "statuses_url": "https://api.github.com/repos/mbj/mrs/statuses/abc123",
                        "number": 123,
                        "state": "open",
                        "locked": false,
                        "title": "Test PR",
                        "user": {"login": "mbj"},
                        "body": null,
                        "labels": [],
                        "milestone": null,
                        "active_lock_reason": null,
                        "created_at": "2024-01-01T00:00:00Z",
                        "updated_at": "2024-01-01T00:00:00Z",
                        "closed_at": null,
                        "merged_at": null,
                        "merge_commit_sha": null,
                        "assignee": null,
                        "assignees": [],
                        "requested_reviewers": [],
                        "requested_teams": [],
                        "head": {
                            "label": "mbj:feature",
                            "ref": "feature",
                            "sha": "abc123abc123abc123abc123abc123abc123abc1",
                            "user": {"login": "mbj"},
                            "repo": {"name": "mrs"}
                        },
                        "base": {
                            "label": "mbj:main",
                            "ref": "main",
                            "sha": "def456def456def456def456def456def456def4",
                            "user": {"login": "mbj"},
                            "repo": {"name": "mrs"}
                        },
                        "_links": {},
                        "author_association": "OWNER",
                        "auto_merge": null,
                        "draft": false
                    }
                ]"#,
            );

            let result = ListPullRequests::DECODER.decode(response).await.unwrap();

            assert_eq!(
                result,
                mhttp::link::Paginated {
                    data: vec![PullRequestInfo {
                        id: 1,
                        url: "https://api.github.com/repos/mbj/mrs/pulls/123"
                            .parse()
                            .unwrap(),
                        html_url: "https://github.com/mbj/mrs/pull/123".parse().unwrap(),
                        diff_url: "https://github.com/mbj/mrs/pull/123.diff".parse().unwrap(),
                        patch_url: "https://github.com/mbj/mrs/pull/123.patch".parse().unwrap(),
                        issue_url: "https://api.github.com/repos/mbj/mrs/issues/123"
                            .parse()
                            .unwrap(),
                        commits_url: "https://api.github.com/repos/mbj/mrs/pulls/123/commits"
                            .parse()
                            .unwrap(),
                        review_comments_url:
                            "https://api.github.com/repos/mbj/mrs/pulls/123/comments"
                                .parse()
                                .unwrap(),
                        review_comment_url:
                            "https://api.github.com/repos/mbj/mrs/pulls/comments{/number}"
                                .parse()
                                .unwrap(),
                        comments_url: "https://api.github.com/repos/mbj/mrs/issues/123/comments"
                            .parse()
                            .unwrap(),
                        statuses_url: "https://api.github.com/repos/mbj/mrs/statuses/abc123"
                            .parse()
                            .unwrap(),
                        number: 123,
                        state: PullRequestState::Open,
                        locked: false,
                        title: "Test PR".to_string(),
                        user: serde_json::json!({"login": "mbj"}),
                        body: None,
                        labels: serde_json::json!([]),
                        milestone: None,
                        active_lock_reason: None,
                        created_at: "2024-01-01T00:00:00Z".to_string(),
                        updated_at: "2024-01-01T00:00:00Z".to_string(),
                        closed_at: None,
                        merged_at: None,
                        merge_commit_sha: None,
                        assignee: None,
                        assignees: serde_json::json!([]),
                        requested_reviewers: serde_json::json!([]),
                        requested_teams: serde_json::json!([]),
                        head: PullRequestRef {
                            label: "mbj:feature".to_string(),
                            r#ref: "feature".to_string(),
                            sha: "abc123abc123abc123abc123abc123abc123abc1".parse().unwrap(),
                            user: serde_json::json!({"login": "mbj"}),
                            repo: serde_json::json!({"name": "mrs"}),
                        },
                        base: PullRequestRef {
                            label: "mbj:main".to_string(),
                            r#ref: "main".to_string(),
                            sha: "def456def456def456def456def456def456def4".parse().unwrap(),
                            user: serde_json::json!({"login": "mbj"}),
                            repo: serde_json::json!({"name": "mrs"}),
                        },
                        links: serde_json::json!({}),
                        author_association: "OWNER".to_string(),
                        auto_merge: None,
                        draft: false,
                    }],
                    links: None,
                }
            );
        }
    }

    mod list_pull_request_commits {
        use super::*;

        #[test]
        fn request_builder() {
            let request = ListPullRequestCommits {
                repository: "mbj/mrs".parse().unwrap(),
                pull_request: "123".parse().unwrap(),
            };

            TestRequest {
                method: reqwest::Method::GET,
                url: "https://api.github.com/repos/mbj/mrs/pulls/123/commits?per_page=100"
                    .parse()
                    .unwrap(),
                headers: http::HeaderMap::new(),
                body: None,
            }
            .assert(&request, &base_url());
        }

        #[tokio::test]
        async fn decoder() {
            let response = json_response(
                http::StatusCode::OK,
                r#"[
                    {
                        "url": "https://api.github.com/repos/mbj/mrs/commits/abc123",
                        "sha": "abc123abc123abc123abc123abc123abc123abc1",
                        "node_id": "C_abc123",
                        "html_url": "https://github.com/mbj/mrs/commit/abc123",
                        "comments_url": "https://api.github.com/repos/mbj/mrs/commits/abc123/comments",
                        "commit": {
                            "url": "https://api.github.com/repos/mbj/mrs/git/commits/abc123",
                            "author": {
                                "name": "Test Author",
                                "email": "author@example.com",
                                "date": "2024-01-01T00:00:00Z"
                            },
                            "committer": {
                                "name": "Test Committer",
                                "email": "committer@example.com",
                                "date": "2024-01-01T00:00:00Z"
                            },
                            "message": "Test commit",
                            "comment_count": 0,
                            "tree": {
                                "url": "https://api.github.com/repos/mbj/mrs/git/trees/tree123",
                                "sha": "1234567890123456789012345678901234567890"
                            },
                            "verification": {
                                "verified": false,
                                "reason": "unsigned",
                                "signature": null,
                                "payload": null
                            }
                        },
                        "author": {"login": "mbj"},
                        "committer": {"login": "mbj"},
                        "parents": [
                            {
                                "url": "https://api.github.com/repos/mbj/mrs/commits/parent123",
                                "sha": "aaaaaaaabbbbbbbbccccccccddddddddeeeeeeee",
                                "html_url": "https://github.com/mbj/mrs/commit/parent123"
                            }
                        ]
                    }
                ]"#,
            );

            let result = ListPullRequestCommits::DECODER
                .decode(response)
                .await
                .unwrap();

            assert_eq!(
                result,
                mhttp::link::Paginated {
                    data: vec![PullRequestCommit {
                        url: "https://api.github.com/repos/mbj/mrs/commits/abc123"
                            .parse()
                            .unwrap(),
                        sha: "abc123abc123abc123abc123abc123abc123abc1".parse().unwrap(),
                        node_id: "C_abc123".to_string(),
                        html_url: "https://github.com/mbj/mrs/commit/abc123".parse().unwrap(),
                        comments_url:
                            "https://api.github.com/repos/mbj/mrs/commits/abc123/comments"
                                .parse()
                                .unwrap(),
                        commit: CommitDetails {
                            url: "https://api.github.com/repos/mbj/mrs/git/commits/abc123"
                                .parse()
                                .unwrap(),
                            author: GitUser {
                                name: "Test Author".to_string(),
                                email: "author@example.com".to_string(),
                                date: "2024-01-01T00:00:00Z".to_string(),
                            },
                            committer: GitUser {
                                name: "Test Committer".to_string(),
                                email: "committer@example.com".to_string(),
                                date: "2024-01-01T00:00:00Z".to_string(),
                            },
                            message: "Test commit".to_string(),
                            comment_count: 0,
                            tree: CommitTree {
                                url: "https://api.github.com/repos/mbj/mrs/git/trees/tree123"
                                    .parse()
                                    .unwrap(),
                                sha: "1234567890123456789012345678901234567890".parse().unwrap(),
                            },
                            verification: CommitVerification {
                                verified: false,
                                reason: "unsigned".to_string(),
                                signature: None,
                                payload: None,
                            },
                        },
                        author: Some(serde_json::json!({"login": "mbj"})),
                        committer: Some(serde_json::json!({"login": "mbj"})),
                        parents: vec![CommitParent {
                            url: "https://api.github.com/repos/mbj/mrs/commits/parent123"
                                .parse()
                                .unwrap(),
                            sha: "aaaaaaaabbbbbbbbccccccccddddddddeeeeeeee".parse().unwrap(),
                            html_url: "https://github.com/mbj/mrs/commit/parent123"
                                .parse()
                                .unwrap(),
                        }],
                    }],
                    links: None,
                }
            );
        }
    }
}

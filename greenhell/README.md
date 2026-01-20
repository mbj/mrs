# greenhell - GitHub Status Check Aggregator

> **Experimental**: This project is under development. The README describes the planned architecture and features, not all of which are implemented yet.

A GitHub status check that ensures all commits in a PR pass CI, not just the latest one.

## Name

Named after the Nürburgring Nordschleife, nicknamed "Die Grüne Hölle" (The Green Hell) by Jackie Stewart. The name carries a double meaning: all commits must be green, and the demanding gauntlet they must pass through.

## Problem

When multiple commits are pushed to a branch, CI runs on each commit independently. The PR only shows the status of the latest commit. If an earlier commit fails CI but the latest passes, the PR appears green even though there's a broken commit in the history.

## Solution

greenhell creates a status check on the latest commit that:

1. Finds all commits since the branch diverged from main
2. Waits for all status checks on those commits to complete
3. Passes only if all commits pass
4. Fails if any commit fails
5. Automatically passes if superseded by a new commit (only the latest matters)

## Architecture

### Deployment Modes

**CLI** (for testing/manual runs):
```bash
# Discover and print GitHub token
greenhell cli-token

# Evaluate by branch
greenhell evaluate --repository owner/repo --branch feature-xyz

# Evaluate by PR number
greenhell evaluate --repository owner/repo --pull-request 123

# Evaluate all open PRs
greenhell evaluate-all owner/repo
```

CLI Options:
| Option                      | Purpose                                                          |
|-----------------------------|------------------------------------------------------------------|
| `--repository <owner/repo>` | Target repository                                                |
| `--branch <name>`           | Branch to evaluate (evaluate only)                               |
| `--pull-request <number>`   | PR to evaluate (evaluate only, mutually exclusive with --branch) |
| `--dry-run`                 | Print actions without executing                                  |

Token resolution (in order):
1. `GH_TOKEN` env var
2. `GITHUB_TOKEN` env var
3. `gh auth token` (shells out to gh CLI if installed)

**AWS Lambda** (production):
```
GitHub ──webhooks──▶ Lambda Function URL ──▶ Lambda
                                                │
EventBridge (1 min) ───────────────────────────▶│
                                                │
                                                └── GitHub API
```

### Design Principles

- **Stateless**: No database required. All state is derived from GitHub API on each invocation.
- **Hybrid triggers**: Webhooks for real-time updates, scheduled reconciliation (1 min) to catch missed webhooks.
- **GitHub App auth**: Uses GitHub App for Checks API access (richer output than Statuses API).

### Webhook Events

- `push` - New commits pushed, create/update checks
- `status` - Commit status updates
- `check_run` - Check run updates
- `check_suite` - Check suite updates

### GitHub API Endpoints

Implemented directly (no SDK):

- `GET /repos/{owner}/{repo}/compare/{base}...{head}` - Get commits since main
- `GET /repos/{owner}/{repo}/commits/{ref}/status` - Get combined status
- `GET /repos/{owner}/{repo}/commits/{ref}/check-runs` - List check runs
- `POST /repos/{owner}/{repo}/check-runs` - Create check run
- `PATCH /repos/{owner}/{repo}/check-runs/{id}` - Update check run
- `GET /repos/{owner}/{repo}/pulls?state=open` - List open PRs (for reconciliation)

### Authentication

GitHub App authentication flow:
1. Generate JWT from App private key (RS256)
2. Exchange JWT for installation access token
3. Use installation token for API calls

Webhook verification:
- Verify `X-Hub-Signature-256` header using HMAC-SHA256 with webhook secret

### GitHub App Setup

The official GitHub App name is `greenhell-bot`. Three integration options are supported:

**Option A: Self-Hosted**
- Create your own GitHub App with the required permissions
- Deploy your own Lambda and infrastructure
- Provide App ID, private key, and webhook secret to greenhell
- Full control over the App and deployment

**Option B: Public App (hosted)**
- Install the public `greenhell-bot` App with one click
- Select which repositories to grant access
- Minimal setup, hosted webhook endpoint
- Currently limited to friends and family (hardcoded repo allowlist). Contact the author if interested.

**Option C: Manifest Flow (hosted)**
- Click a link to auto-create a pre-configured GitHub App in your account
- Permissions and webhooks are pre-filled
- You get your own App with minimal effort
- Currently limited to friends and family (hardcoded repo allowlist). Contact the author if interested.

#### Required Permissions

| Permission      | Level        | Purpose                            |
|-----------------|--------------|------------------------------------|
| Checks          | Read & Write | Create/update check runs           |
| Contents        | Read         | Compare commits since main         |
| Pull Requests   | Read         | List open PRs for reconciliation   |
| Commit Statuses | Read         | Read status of other CI systems    |
| Metadata        | Read         | Required (implicit)                |

#### Webhook Subscriptions

- `push`
- `status`
- `check_run`
- `check_suite`

## Future Improvements

- **Stale webhook detection**: When GitHub recovers from an outage, it delivers a backlog of webhooks that were queued during downtime. Due to the hybrid architecture, this does not cause functional problems - the scheduled reconciliation ensures correctness regardless. However, it creates excessive API call volumes. We will detect and ignore stale webhooks in the future.
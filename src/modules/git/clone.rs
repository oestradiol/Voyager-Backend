use axum::http::StatusCode;
use git2::{
  build::{self, RepoBuilder},
  RemoteCallbacks, Repository,
};
use std::path::Path;
use tracing::{event, Level};

use crate::{
  configs::environment::{GITHUB_ORG_NAME, GITHUB_PAT},
  types::other::voyager_error::VoyagerError,
  utils::Error,
};

pub fn clone(
  repo_url: &str,
  repo_branch: Option<String>,
  repo_path: &Path,
) -> Result<Repository, VoyagerError> {
  let name = GITHUB_ORG_NAME.as_ref();
  if !repo_url.starts_with(name) {
    return Err(VoyagerError::repository_not_owned(repo_url));
  }

  let pat = GITHUB_PAT.as_ref();
  let repo_url = format!("https://github.com/{repo_url}.git");

  // Configure authentication
  let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|_url, _username_from_url, _allowed_types| {
    git2::Cred::userpass_plaintext(name, pat)
  });

  // Prepare fetch options.
  let mut fo = git2::FetchOptions::new();
  fo.remote_callbacks(callbacks);

  // Prepare builder.
  let mut builder = git2::build::RepoBuilder::new();
  builder.fetch_options(fo);

  // Set branch
  if let Some(branch) = repo_branch {
    builder.branch(&branch);
  }

  // Clone
  builder
    .clone(&repo_url, repo_path)
    .map_err(|e| VoyagerError::clone(Box::new(e)))
}

impl VoyagerError {
  fn repository_not_owned(repo_url: &str) -> Self {
    let name: &str = GITHUB_ORG_NAME.as_ref();
    Self::new(
      format!("Repository {repo_url} is not owned by {name}"),
      StatusCode::BAD_REQUEST,
      None,
    )
  }

  fn clone(e: Error) -> Self {
    Self::new(
      "Failed to clone git repository!".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}

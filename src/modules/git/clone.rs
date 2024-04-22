use axum::http::StatusCode;
use git2::{
  RemoteCallbacks, Repository,
};
use std::path::Path;
use tracing::{event, Level};

use crate::{
  configs::environment::{GIT_PAT, GIT_USERNAME},
  types::other::voyager_error::VoyagerError,
  utils::Error,
};

pub fn clone(
  repo_url: &str,
  repo_branch: Option<String>,
  repo_path: &Path,
) -> Result<Repository, VoyagerError> {
  event!(Level::INFO, "Cloning repository: {}", repo_url);

  let username = GIT_USERNAME.as_ref();

  let pat = GIT_PAT.as_ref();
  let repo_url = format!("https://git.lunarlabs.cc/{repo_url}.git");

  // Configure authentication
  let mut callbacks = RemoteCallbacks::new();
  callbacks.credentials(|_url, _username_from_url, _allowed_types| {
    git2::Cred::userpass_plaintext(username, pat)
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
  let result = builder
    .clone(&repo_url, repo_path)
    .map_err(|e| VoyagerError::clone(Box::new(e)));

  event!(Level::DEBUG, "Done cloning repository.");

  result
}

impl VoyagerError {
  fn clone(e: Error) -> Self {
    Self::new(
      "Failed to clone git repository!".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}

use git2::{
  build::{self, RepoBuilder},
  RemoteCallbacks,
};
use std::path::Path;
use tracing::{event, Level};

use crate::configs::environment::{GITHUB_ORG_NAME, GITHUB_PAT};

// fn clone(repo_url: &str) -> Option<String> {
//   let name = GITHUB_ORG_NAME.as_ref();
//   if !repo_url.starts_with(name) {
//     event!(Level::WARN, "Repository is not owned by {name}");
//     return None;
//   }
//
//   let pat = GITHUB_PAT.as_ref();
//   let repo_url = format!("https://github.com/{repo_url}.git");
//   let repo_path = Path::new("path/to/clone/repository");
//
//   // Configure authentication
//   let mut callbacks = RemoteCallbacks::new();
//   callbacks.credentials(|_url, _username_from_url, _allowed_types| {
//     git2::Cred::userpass_plaintext(name, pat)
//   });
//
//   // Clone the repository
//   let mut opts = build::RepoBuilder::new();
//   opts.remote_callbacks(callbacks);
//   match RepoBuilder::new().clone(repo_url, repo_path) {
//     Ok(_) => println!("Repository cloned successfully!"),
//     Err(e) => eprintln!("Failed to clone repository: {}", e),
//   }
// }

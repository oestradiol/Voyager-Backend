# Roadmap


## Optimizations
- When requesting a new deployment, check if old docker build for this already exists and if the git commit hash used for that one is the same as the new one to prevent using extra resources to build a new docker image.
- ??? put any more optimizations we can do to both the website that we are deploying and the entire deployment process.

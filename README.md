# Voyager API Endpoints Description

All endpoints must be accessed with the X-API-Key header:
- X-API-Key: <your-api-key>

## /deployment/deploy (POST)
Deploys a container

Request query parameters:
- *repoUrl: The repository URL
- subdomain: Subdomain to be deployed to (if empty, will attempt to deploy to https://pinkcloud.studio)
- *mode: Mode to be deployed (either 'preview' or 'production')

* = required variables

Example:

curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/deploy?repoUrl=PinkCloudStudios/MyDeployment&subdomain=my-deployment&mode=preview \
    --header 'X-Api-Key: 123123123abcabcabc'


Response content type is application/json and is of format:

{
    code: Int,
    message: String,
    errors: Array<String>,
    id: String?
}

Example:

Status Code: 200 (OK)
Response body:
{
    code: 200,
    message: "Deployed",
    errors: [],
    id: "349j4h52jk3h55hc1fjl2kh"
}

Status code: 403 (Forbidden)
Response body:
{
    code: 403
    message: "Deployment already exists"
    errors: ["Deployment already exists"]
    id: null
}


## /deployment/{id}/logs (POST)
Gets the logs from a container

Path Variables:
- id: The deployment id

Example

curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/48h2jk3h43/logs \
    --header 'X-API-Key: 123123abcabc'


Response content type is of application/json and is of format:

{
    code: Int,
    message: String,
    errors: Array<String>
    logs: Array<String>?
}

Example:

Status Code: 200 (OK):
Response body:
{
    code: 200,
    message: "Logs retrieved",
    errors: [],
    logs: ["Starting container..", "Done!"]
}

Status Code: 404 (Not Found):
Response body:
{
    code: 404,
    message: "Deployment not found",
    errors: ["Deployment not found"],
    logs: null
}


## /deployment/{id}/stop
Stops and removes the deployment

Path Variables:
- id: The deployment id

Example:

curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/48h2jk3h43/stop \
    --header 'X-API-Key: 123123abcabc'


Response content type is of application/json and is of format:

{
    code: Int,
    message: String,
    errors: Array<String>
}

Example:

Status Code: 200 (OK):
Response body:
{
    code: 200,
    message: "Deployment deleted, id is no longer valid",
    errors: []
}

Status Code 500 (Internal Server Error):
Response body:
{
    code: 500
    message: "Internal Server Error",
    errors: ["Internal Server Error", "Could not stop docker container: <insert-random-error>"]
}
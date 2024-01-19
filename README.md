# Voyager API Endpoints Description

All endpoints must be accessed with the X-API-Key header:
- X-API-Key: &lt;your-api-key&gt;

## /deployment/deploy (POST)
Deploys a container

Request query parameters:
- *repoUrl: The repository URL
- subdomain: Subdomain to be deployed to (if empty, will attempt to deploy to https://pinkcloud.studio)
- *mode: Mode to be deployed (either 'preview' or 'production')

Example:

<pre>
curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/deploy?repoUrl=PinkCloudStudios/MyDeployment&subdomain=my-deployment&mode=preview \
    --header 'X-Api-Key: 123123123abcabcabc'
</pre>

Response content type is application/json and is of format:
<pre>
{
    code: Int,
    message: String,
    errors: Array&lt;String&gt;,
    id: String?
}
</pre>

Example:

<pre>
Status Code: 200 (OK)
Response body:
{
    code: 200,
    message: "Deployed",
    errors: [],
    id: "349j4h52jk3h55hc1fjl2kh"
}
</pre>

<pre>
Status code: 403 (Forbidden)
Response body:
{
    code: 403
    message: "Failed"
    errors: ["Deployment already exists"]
    id: null
}
</pre>


## /deployment/{id}/logs (POST)
Gets the logs from a container

Path Variables:
- *id: The deployment id

Example

<pre>
curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/48h2jk3h43/logs \
    --header 'X-API-Key: 123123abcabc'
</pre>

Response content type is of application/json and is of format:

<pre>
{
    code: Int,
    message: String,
    errors: Array<String>
    logs: Array<String>?
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    code: 200,
    message: "Logs Retrieved",
    errors: [],
    logs: ["Starting container..", "Done!"]
}
</pre>

<pre>
Status Code: 404 (Not Found):
Response body:
{
    code: 404,
    message: "Failed",
    errors: ["Deployment not found"],
    logs: null
}
</pre>

## /deployment/{id}/stop
Stops and removes the deployment

Path Variables:
- *id: The deployment id

Example:

<pre>
curl --request POST \
    --url https://voyager-api.pinkcloud.studio/deployment/48h2jk3h43/stop \
    --header 'X-API-Key: 123123abcabc'
</pre>


Response content type is of application/json and is of format:

<pre>
{
    code: Int,
    message: String,
    errors: Array<String>
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    code: 200,
    message: "Success",
    errors: []
}
</pre>

<pre>
Status Code 500 (Internal Server Error):
Response body:
{
    code: 500
    message: "Internal Server Error",
    errors: ["Could not stop docker container: <insert-random-error>"]
}
</pre>

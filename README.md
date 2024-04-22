# Voyager API Endpoints Description

All endpoints must be accessed with the X-API-Key header:
- X-API-Key: &lt;your-api-key&gt;

## /deployment (POST)
Deploys a container

Request query parameters:
- *repoUrl: The repository URL (and optional branch followed after an '@')
- subdomain: Subdomain to be deployed to (if empty, will attempt to deploy to https://pinkcloud.studio)
- *mode: Mode to be deployed (either 'preview' or 'production')

Example:

<pre>
curl --request POST \
    --url "https://voyager-api.pinkcloud.studio/deployment?repoUrl=PinkCloudStudios/MyDeployment@MyBranch&subdomain=my-deployment&mode=preview" \
    --header 'X-Api-Key: 123123123abcabcabc'
</pre>

Response content type is application/json and is of format:

<pre>
{
    logs: {
        message: string,
        errors: array[string],
    },
    id: string or null
}
</pre>

Example:

<pre>
Status Code: 200 (OK)
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "id": "f7ea72e3-9c8e-40ef-8464-18b732667c38"
}
</pre>

<pre>
Status code: 403 (Forbidden)
Response body:
{
    "logs": {
        "message": "Failed to create deployment",
        "errors": ["Deployment already exists"]
    },
    "id": null
}
</pre>


## /deployments/{id}/logs (GET)
Gets the logs from a container

Path Variables:
- *id: The deployment id

Example

<pre>
curl --request GET \
    --url "https://voyager-api.pinkcloud.studio/deployments/f7ea72e3-9c8e-40ef-8464-18b732667c38/logs" \
    --header 'X-API-Key: 123123abcabc'
</pre>

Response content type is of application/json and is of format:

<pre>
{
    logs: {
        message: string,
        errors: array[string],
    },
    deployment_logs: array[string] or null
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "deployment_logs": ["Starting container..", "Done!"]
}
</pre>

<pre>
Status Code: 404 (Not Found):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "deployment_logs": null
}
</pre>


## /deployments/{id} (DELETE)
Stops and removes the deployment

Path Variables:
- *id: The deployment id

Example:

<pre>
curl --request DELETE \
    --url "https://voyager-api.pinkcloud.studio/deployments/f7ea72e3-9c8e-40ef-8464-18b732667c38" \
    --header 'X-API-Key: 123123abcabc'
</pre>


Response content type is of application/json and is of format:

<pre>
{
    logs: {
        message: string,
        errors: array[string],
    }
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    }
}
</pre>

<pre>
Status Code 500 (Internal Server Error):
Response body:
{
    "logs": {
        "message": "Failed to stop container",
        "errors": ["Could not stop docker container: <insert-random-error>"]
    },
}
</pre>


## /deployments/{id} (GET)
Gets information about a deployment

Path Variables:
- *id: The deployment id

Example:

<pre>
curl --request GET \
    --url "https://voyager-api.pinkcloud.studio/deployments/f7ea72e3-9c8e-40ef-8464-18b732667c38" \
    --header 'X-API-Key: 123123abcabc'
</pre>


Response content type is of application/json and is of format:

<pre>
{
    logs: {
        message: string,
        errors: array[string],
    },
    deployment: object {
        id: string,
        containerId: string,
        port: integer,
        dnsRecordId: string,
        mode: string,
        host: string,
        state: string,
        directory: string,
        repoUrl: string,
        branch: string,
        createdAt: integer,
    } or null
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "deployment": {
        "id": "f7ea72e3-9c8e-40ef-8464-18b732667c38",
        "containerId": "j4f1iojf1i2kj4e1lkj",
        "port": 34395,
        "dnsRecordId": "jf2398rilhjfsklfh254",
        "mode": "preview",
        "host": "test-preview.pinkcloud.studio",
        "state": "DEPLOYED",
        "directory": "test-preview-default",
        "repoUrl": "test",
        "branch": "main",
        "createdAt": 1706153432 
    }
}
</pre>

<pre>
Status Code 404 (Not Found):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "deployment": null
}
</pre>


## /deployment (GET)
Lists all deployments

Request query parameters:
- repoUrl: specify a repository url to search for (optional)
- branch: specify a branch to search for (optional)

Example:

<pre>
curl --request GET \
    --url "https://voyager-api.pinkcloud.studio/deployment?repoUrl=PinkCloudStudios/my-repo" \
    --header 'X-API-Key: 123123abcabc'
</pre>


Response content type is of application/json and is of format:

<pre>
{
    logs: {
        message: string,
        errors: array[string],
    },
    deployments: array[
        object {
            id: string,
            containerId: string,
            port: integer,
            dnsRecordId: string,
            mode: string,
            host: string,
            state: string,
            directory: string,
            repoUrl: string,
            branch: string,
            createdAt: integer,
        }
    ]
}
</pre>

Example:

<pre>
Status Code: 200 (OK):
Response body:
{
    "logs": {
        "message": "Success!",
        "errors": [],
    },
    "deployments": [
        {
            "id": "f7ea72e3-9c8e-40ef-8464-18b732667c38",
            "containerId": "j4f1iojf1i2kj4e1lkj",
            "port": 34395,
            "dnsRecordId": "jf2398rilhjfsklfh254",
            "mode": "preview",
            "host": "test-preview.pinkcloud.studio",
            "state": "DEPLOYED",
            "directory": "test-preview-default",
            "repoUrl": "test",
            "branch": "main",
            "createdAt": 1706153432 
        },
        {
            "id": "e1f67af8-5d5c-42c9-a144-4198c26197c8",
            "containerId": "d4gu23984guy123894gu",
            "port": 7418,
            "dnsRecordId": "dfhjffjm2u834",
            "mode": "production",
            "host": "test.pinkcloud.studio",
            "state": "DEPLOYED",
            "directory": "test-branch2",
            "repoUrl": "test",
            "branch": "branch2",
            "createdAt": 1706156534 
        }
    ]
}
</pre>

<pre>
Status Code 500 (Internal Server Error):
Response body:
{
    "logs": {
        "message": "Failed to retrieve deployments",
        "errors": ["Bla bla bla"],
    },
    "deployments": []
}
</pre>

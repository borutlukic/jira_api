# jira_api

Rust client library for the Jira DataCenter 11003 REST API.

The generated code in `src/generated/` and the OpenAPI spec in `openapi/` are both committed to the repo.

The spec is sourced from:
```
https://dac-static.atlassian.com/server/jira/platform/jira_software_dc_11003_swagger.v3.json
```

To regenerate `src/generated/` after changing the spec fixes in `codegen/`:

```
make generate
```

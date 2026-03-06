# API Conventions

{{ project_name }} API design standards.

## Response Envelope

```json
// Success
{ "data": { ... } }

// Error
{ "error": { "code": "ERR_CODE", "message": "..." } }
```

## HTTP Status Codes

| Code | Usage |
|------|-------|
| 200  | Success |
| 201  | Created |
| 400  | Bad Request |
| 401  | Unauthorized |
| 403  | Forbidden |
| 404  | Not Found |
| 429  | Rate Limited |
| 500  | Internal Error |

## Authentication

```
Authorization: Bearer <jwt_token>
```

## Endpoints

| Method | Path | Description | Auth |
|--------|------|-------------|------|

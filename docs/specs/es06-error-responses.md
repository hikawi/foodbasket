---
title: ES06 - Error Responses
parent: Product & Engineering Specifications
---

## Revisions

- Initial version v1 (March 1, 2026).

## Summary

This document outlines the format of error responses returned by the backend API.

> This applies to the Rust version of the backend. At the time of writing this documentation,
> this error formatting has not been applied to the Go version, but occurred as
> a side effect of using `Result` types in Rust.

## Error Responses

All errors returned from the backend should be accompanied with a HTTP Status Code,
a Code ID, and a simple message to explain the code:

```json
{
  "status": 500,
  "code": "INTERNAL_SERVER_ERROR",
  "message": "Something went wrong"
}
```

The table below serves as the source of truth for error codes.

| Code                    | Description                                                   |
| ----------------------- | ------------------------------------------------------------- |
| `VALIDATION_FAILED`     | Request body, path or query validation failed.                |
| `WRONG_PASSWORD`        | The password inputted was wrong.                              |
| `USER_NOT_FOUND`        | The requested user could not be found.                        |
| `USER_NO_PASSWORD`      | The requested account does not use passwords to authenticate. |
| `USER_ALREADY_EXISTS`   | There is already a user with the used email.                  |
| `UNAUTHENTICATED`       | The endpoint is protected, and the user is not authenticated. |
| `INTERNAL_SERVER_ERROR` | Generic internal server error. The server is at fault.        |

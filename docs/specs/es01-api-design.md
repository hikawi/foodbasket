---
title: ES01 - API Contracts
parent: Product & Engineering Specifications
---

# ES01 - API Contracts

## Summary

This document aims to provide a standard for API endpoints, created for the
application as a whole to align all developers (me, myself and I) with conformity
to proper boundaries and standards. I absolutely despise tRPC, oRPC or similar
libraries.

## Contract Details

All APIs should honor the following design:

1. All endpoints should begin with their major version. For example, `/v1/endpoint`.
2. All changes to endpoints must be only additions or redirections. Removals or
   changes are destructive functions that have a greater blast radius.
3. All HTTP verbs should be adhered to the RFC Semantic Standards:
   1. All `GET`, `HEAD`, `OPTIONS`, and `TRACE` methods have to be safe. A safe
      method when called, the requester will expect that the server does not
      mutate any meaningful business states on the server, essentially a
      read-only method.
   2. All PUT and UPDATE methods have to be **idempotent** (also all safe methods
      are idempotent). Idempotence requires that multiple requests sent to an
      endpoint have the same mutating effect as one request. The client is not
      expected to retry any non-idempotent requests, and proxies should not retry
      any non-idempotent requests.
4. Responses should be meaningful, with flexible error messages as well as
   meaningful status codes. (200 for good and 400 for bad is not it, and nobody
   listens in school)

There are 3 main types of content type to parse (from frontend to backend):

- `application/json`: Simple JSON-encoded content. Most versatile and
  commonly-used form of communication, but knowing that some people like to be
  complicated, we should use the “4th” content type.
- `application/x-www-form-urlencoded`: Percent-encoded form values in the URL,
  default form for HTML forms. This is usable too, depending on what you want to
  do, but not as smooth as it is for React to use.
- `multipart/form-data`: Content type when you need to send a mix of content types
  in the request. For example, sending both text and images in one request. We can
  also mitigate this by having different endpoints.
- `application/octet-stream`: Arbitrary stream of unknown binary data.

The main response content types should be either XML form or JSON form, XML is discouraged.

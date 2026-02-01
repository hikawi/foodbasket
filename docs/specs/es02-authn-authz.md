---
title: ES02 - Authentication and Authorization
parent: Product & Engineering Specifications
---

# ES02 - Authentication and Authorization

## Summary

This document outlines the authentication and authorization system of the platform
to satisfy a few functional requirements across the platform as a whole.

## Revision History

None yet.

## Objective

This document aims to design an authN and authZ system that completes the following
requirements:

1. The user must be able to share their account between the home page, the tenant
   page and the POS system.
2. The security measures must be up to date, and reasonable to be upheld by browsers,
   such as we can not put too much information in a browser's cookie.
3. The user must be able to authenticate with email address and password.
4. The authentication system must be able to isolate each tenant's context, so that
   each user only ever retrieves data for what they are authorized to interact with.
5. Each role is scoped to tenants.
6. Permissions are global, and static.

### Out of Scope

- OAuth2 or social login (Google, etc.)
- Multi-factor authentication (MFA)
- Session revocation across all devices on password change
- Passwordless or magic links
- Email verification
- Profile pictures

## Authentication and Authorization Flows

This project would go on to use session-based authentication over JWTs due to the
following reasons:

- There are multiple frontends, but only one backend.
- The amount of data needed to cache for a certain user at a time, such as roles
  in many tenants, is usually a big too large to put inside a proper JWT in cookies,
  so I think it's probably better to use a Redis-like caching system, and populate
  the Backend's Context based on the Host header for the multi-tenant architecture.

### Session Rotation

A bit different than JWT key pair, this uses a session sliding mechanism that follows
the following requirements:

1. Every new request to the server, the user's session gets extended by a small
   window of time. (30 minutes)
2. Every privilege escalation should have the session rotated, by creating a new
   session ID, and copy all existing data from the old session over. A privilege
   escalation is considered for example logging in or registering.
3. A new session TTL should be long enough for systems, as to cover an employee's
   shift to work for the use case of a "quiet" shift, without having to re-login
   to the system to serve a customer walking in. This can be ignored if the rotation
   is done automatically on expiry on the backend side, as requirement 2 should
   be able to mitigate this use case also.
4. Each session data should be nested arbitrarily, such as a JSON or a map record,
   to account for multi-tenant level of data and authorization.
5. Session context middlewares should only populate exactly what is needed. A user
   can have permissions on tenant A and B, but if they are on A's system, do not
   populate Echo's context with B's information.
6. A guest session is also implicitly created by the user visiting any of the
   tenants' endpoints, including custom endpoints (but this is out of scope for
   now), to store stuff that might be of use. But currently there are no guest
   data yet. This may be reiterated once we start the use case for anonymous
   ordering, such as reservations, orders, or QR menus.
7. Logging out should instantly revoke the session ID.
8. Each session rotation should provide a grace period of 30 seconds, with a key,
   like `rotated_to` to allow in-flight requests to still be valid for a short while.

### Authentication Context

A user's account should be based on a user's email address as the primary factor.

- When a POS manager wants to assign a new employee, it should be able to link
  to the employee's personal email so they can switch contexts between tenants
  smoothly without relogging.
- A user should be able to be soft-deleted with an email field set to `NULL` to
  preserve business data, even if the user wants to be deleted off the platform.
- Foreign keys should be properly indexed, with cache lines or denormalization
  to a REASONABLE degree to ensure performance. Do not fully normalize too much,
  but do not fully denormalize. Only requirement is that core business entities
  require normalization. How much else is based on the developers.
- There needs a method to see how many sessions a user can have for a use case
  "Log out on all devices".

### Valkey Key Naming

Since we will be using a universal cache database in production between multiple
different apps as all apps are considered proofs of concept with not too much
data. Therefore, all keys used should have this project's name prefixed: `foodbasket`.

- Sessions should be named as `foodbasket:auth:sess:<session_id>`
- User sessions index should be named as `foodbasket:auth:user_idx:<user_id>`

> Further key naming related to authentication may be added in the future.

### API Design

For main principles of API Design, check out [ES01](./es01-api-design.md).

- `/auth/register`: Registers a new account, instantly rotating a session.
  - If the user is currently a guest, do a _privilege escalation_.
  - If the user is currently logged in as another account, simply rotate
    the session to the other account without any copy.
- `/auth/login`: Logins to an existing account.
  - If the user is currently a guest, do a _privilege escalation_.
  - If the user is currently logged in as another account, simply rotate
    the session to the other account without any copy.
- `/auth/logout`: Logouts an existing account.
  - Revokes instantly the session, any in-flight requests are considered
    dropped to prevent additional changes that should not be saved after a
    deliberate logout.
  - Clears the cookie browser-side.

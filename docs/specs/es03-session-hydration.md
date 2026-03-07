---
title: ES03 - Host, Tenant and Session Hydration
parent: Product & Engineering Specifications
---

# ES03 - Host, Tenant and Session Hydration

## Revisions History

| Version |    Date    | Changelog                                           |
| :-----: | :--------: | --------------------------------------------------- |
|   2.0   | 2026-03-05 | Changes the architecture from Go Echo to Rust Axum. |
|   1.0   | 2026-02-03 | Initial version.                                    |

## Summary

This document outlines the process of 'hydrating' a handler's context throughout
the pipeline of Axum routing.

## Rationale

This process is done to provide the best level of isolation, without having to burden
the client or the API users with a lot of query parameters, as most of them should
be done via a browser. This logic may be changed later, but the logic of injecting
them into handlers would not change.

## The Pipeline

The following pipeline was generated mostly by Gemini after feeding it the code
available in `middlewares.rs`. I have fact checked these (should be up to date)
and added my own edits.

### Layer 1: Dynamic CORS

- **Purpose**: Validates the `Origin` header.
- **Result**: Sets CORS headers. Prevents unauthorized cross-origin browser requests.
  Returns 204 for Preflight OPTIONS runs.
- **Extension**: `OriginUrl` (Enum: Invalid, Valid(String)).

> Warning! This layer does not block against valid origins, but invalid tenants.
> This is the most lightweight layer.

### Layer 2: Origin Hydration

- **Purpose**: Determines "Where" the request is going.
- **Logic**: Parses the Origin to identify if the context is `Admin`, `TenantPos`
  (Staff), or `TenantHome` (Customer).
- **Extension**: `OriginContext` (Enum: Admin, TenantPos(Uuid), TenantHome(Uuid),
  Anonymous).

### Layer 3: Session Hydration

- **Purpose**: Determines "Who" is making the request.
- **Logic**: Extracts `session_id` from Cookies and performs a lookup in **Valkey**.
- **Extension**: `SessionContext` (Enum: Authenticated(User), Anonymous).

### Layer 4: Profile Hydration

- **Purpose**: Connects the "Who" to the "Where."
- **Logic**:
  - If `TenantPos` + `Authenticated` -> Fetches `StaffProfile`.
  - If `TenantHome` + `Authenticated` -> Fetches `CustomerProfile`.
  - Else -> Fetches `SystemProfile`.
- **Extension**: `ProfileContext` (Enum: Staff(Arc), Customer(Arc), System(Arc),
  Anonymous).

### Layer 5: Branch Hydration

- **Purpose**: Scopes the request to a physical location.
- **Logic**: Extracts `X-Branch-ID` header. Verifies via Valkey `SMEMBERS` that
  the branch belongs to the current `TenantID`.
- **Extension**: `BranchContext` (Enum: Branch(Uuid), Anonymous).

### Layer 6: Policy Extraction

- **Purpose**: Fetches the raw security rules.
- **Logic**: Executes a hierarchical SQL query (or cache lookup) based on the
  `Profile` and `Branch` context.
- **Extension**: `PolicyContext` (Enum: Authenticated(Arc<Vec\<Policy\>>), Anonymous).

### Layer 7: Request Solidify

- **Purpose**: Final assembly of the "God Object."
- **Logic**:
  - Consumes all previous extensions.
  - Flattens `Vec<Policy>` into a `HashMap<String, bool>` inside `RequestContext`.
  - Applies **Deny-Overrides-Allow** logic.
- **Extension**: `Arc<RequestContext>`

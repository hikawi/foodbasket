---
title: ES01 - Authentication and Authorization
parent: Product & Engineering Specifications
---

# ES01 - Authentication and Authorization

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

### Assumptions & Threat Model

- Primary threat: unauthorized access to another tenant's data (cross-tenant read/write).
- Attacker model: malicious staff member, compromised customer account, network
  eavesdropper.
- We assume TLS everywhere (HTTPS enforced on production and staging).
- Device thefts are out of scope.
- Rate-limiting and brute-force protection are in scope.

## Database Design

The database design must account for the system

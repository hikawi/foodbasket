---
parent: User Stories
title: US01 - Staff Tenant Scoping
---

**As a restaurant staff member**,
I want to **log in securely** and **automatically be scoped to my assigned tenant/branch**,
so that I **only see and act on data for the restaurant I'm working with.**

## Acceptance Criteria

1. **Happy Path – Single Tenant**
   - Given valid credentials (email + password) and the user has exactly one
     tenant role assigned, when the user submits the login form from a tenant-specific
     POS URL (e.g., `pos.twinbells.foodbasket.luny.dev` or `/twinbells/pos`), then
     authentication succeeds, and an opaque session token is generated and
     stored in Valkey (with TTL), and the session payload includes at minimum:
     `user_id`, `tenant_id`(s), roles for the active tenant, and expiration.
2. **Session Creation & Storage**
   - The session token must be stored as a secure key-value pair in Valkey
     (example key: `pos:session:<token>`).
   - The token must be set as HttpOnly, Secure, SameSite=Strict (or None if
     redirects require it).
   - Session expires after inactivity (e.g., 30–60 minutes – configurable)
     or explicit logout.
3. **Tenant Scoping from URL**
   - Given login occurs from a tenant-specific POS URL (subdomain or path),
     When the session is established, then the active tenant is extracted from
     the host/path and set in the session, and all subsequent API calls and
     database queries are automatically scoped to that `tenant_id` (enforced by
     middleware).
4. **Multi-Tenant User Handling**
   - Given the user has roles in multiple tenants and logs in from a generic POS
     URL (e.g., `pos.foodbasket.luny.dev` or `/pos`), when login succeeds, then
     a simple tenant/branch selector screen is shown (dropdown or cards listing
     only tenants the user has access to), and upon selection, the user is
     redirected to the correct POS URL (e.g., `pos.twinbells.foodbasket.luny.dev`)
     and the session active tenant is updated.
5. **Access Denied Handling**
   - Given valid login credentials but the user has no role in the requested
     tenant (determined from URL), when attempting to access the POS, then the
     user is redirected to a user-friendly error page or the tenant's public menu
     page (e.g., `twinbells.foodbasket.luny.dev`) with a message.
6. **Role-Based UI Adaptation**
   - Given successful login and tenant scoping, when the POS dashboard loads, then
     UI elements are shown or hidden based on the user's roles in the active
     tenant (example: waiters see only the order queue, managers see additional
     links to Menu Editor and Users Editor).
7. **Touch-Friendly & Responsive Login Form**
   - The login form must be usable on tablets (large input fields, visible
     password toggle button, keyboard-friendly).
   - Error messages appear inline (e.g., "Invalid credentials" in red below the
     password field).
   - Form prevents double submission (button disabled after first click).
8. **Security & Negative Cases**
   - Invalid credentials -> clear error message is displayed, no session is
     created, and login attempts are rate-limited (e.g., 5 failed attempts ->
     5-minute lockout).
   - Account inactive/locked -> specific message "Account is inactive –
     contact support" is shown.
   - Passwords are never logged or exposed in error messages or URLs or logs.

## Out-of-scope

- Multi-factor authentication (MFA)
- Password reset / forgot password flow
- Branch-level selection (only tenant for now)
- Real-time session invalidation across devices

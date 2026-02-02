---
title: ES03 - Host, Tenant and Session Hydration
parent: Product & Engineering Specifications
---

# ES03 - Host, Tenant and Session Hydration

## Summary

This document outlines the process of hydrating a context throughout the echo's
pipeline to allow downstream handlers and middlewares to extract tenant-related
information without having to repeatedly query cache or databases.

## Pipeline Diagram

The following diagram was drawn by an LLM.

```plaintext
Client (browser / iPad / POS terminal)
          |
          | HTTPS request
          v
[Public Internet]
          |
          v
    VPS Public IP
          |
          |
          v
Caddy (Docker container, internal on :80)
          |
          |
          v
Echo / Go Backend (Docker container)
          |
          +--> [CORSMiddleware]
          |      - Dynamic CORS middleware to adapt to what current tenants
          |        there are registered with us.
          |      - Uses Valkey for caching.
          |
          +--→ [HostHydrate Middleware]  <--- Reads Host header
          |      - Extracts tenant_id from subdomain (e.g. twinbells)
          |      - Validates domain suffix
          |      - Sets c.Set("tenant_id", "twinbells")
          |      - Sets c.Set("is_pos", true) if prefixed with POS
          |
          +--→ [SessionHydrate Middleware]
          |      - Reads session cookie
          |      - Loads session from Valkey
          |      - Handles stale session here with rotated_to
          |      - Sets c.Set("session", sessionData)
          |
          +--> [TenantHydrate Middleware]
          |      - Reads sessionData from SessionHydrate.
          |      - If not a guest, collect permissions if they are on a tenant's
          |        host or POS host.
          |      - Sets c.Set("permissions", permissions)
          |
          +--> [HasPermission Middleware] (OPTIONAL)
          |      - Reads permissions from TenantHydrate
          |      - Blocks with 403 Forbidden if not enough permissions.
          |
          v
Route Handler (e.g. /menus, /orders, /inventory)
          |
          +--→ Uses c.Get("tenant_id") and c.Get("session")
          |      - Fetches menus / data for the correct tenant
          |      - Applies permissions from session
          |
          v
Response back through Caddy → Nginx → Client
```

The rationale for this is to allow tenant-level scoping in all endpoints, so clients
don't have to keep appending a query parameter like `?tenant_id=edbdf736-1c2d-4035-8e70-81f1afd76bbf`
every time they want to retrieve something, which is rather ugly in my opinion.

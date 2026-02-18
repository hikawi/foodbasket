---
title: ES05 - Tenant Registry
parent: Product & Engineering Specifications
---

# ES05 - Tenant Registry

## Revisions

- Initial version v1 (January 9, 2026).

## Summary

This document should be the source of truth for how the middleware `CORSMiddleware`
and `HostHydrate` should find out whether it is a valid or an invalid tenant ID.

Refer to [ES03 Session Hydration](./es03-session-hydration.md).

## Valkey Layer

This layer should be designed to be as agnostic as possible to allow easy mocking
and easy isolated testing later, but this layer should provide the following procedure
calls to support ES05:

- A way to map a tenant slug to a tenant UUID: such as `twinbells -> 12af89f4-2adc-40d2-a33b-67e20f6fb7b5`.
  Uses the Valkey key `foodbasket:tenants:slug:<slug>` that maps to a UUID string.
  This mapping needs to have a way to mark an absence of such slug.
- A way to map a tenant UUID to some simple metadata in a HSET, such as
  `uuid -> {}`. Uses the Valkey key `foodbasket:tenants:uuid:<uuid>` that maps
  to a map. This mapping needs to have a way to mark an absence of such UUID.

Keys are built in the `constants` package to provide a way to construct Valkey keys
without the overhead of `sprintf` or similar formatting libraries. String Builders
and Concatenations should be the fastest.

## Service Layer

The CORSMiddleware and HostHydrate middlewares need to satisfy the following requirements:

1. CORSMiddleware only returns the requested Origin if and only if, the tenant's
   slug or ID exists.
2. HostHydrate only hydrates the correct Host, provided that the tenant's slug
   or ID exists.

All checks must be done through a service layer to allow easy mocking and swapping.

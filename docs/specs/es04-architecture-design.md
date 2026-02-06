---
title: ES04 - Architecture Design
parent: Product & Engineering Specifications
---

# ES04 - Architecture Design

## Revisions

- Version 2 (January 6, 2026):
  - Changed POS system's domains to better fit wildcard DNS-01.
- Initial version: `v1`
  - Released on January 3, 2026.

## Summary

This document outlines the architecture design of the FoodBasket platform. This
should show a high-level logical overview and some simple user flows on how a
request hits both ends.

## Architecture Design

![Architecture Design Diagram](../images/architecture-design-v2.png)

Includes the following components, truncated `tenant2` for short:

- **Home Platform**: `foodbasket.app`. This provides the landing page to advertise
  the software, our visions and such things for a SaaS. Also includes simple
  SSO across the platform here to easily navigate to each tenant's storefront,
  included as a way for consumers to order from.
- **Tenant's Platform**: `tenant.foodbasket.app`. This is the hosted landing
  page of the restaurant registered with the service. This provides customization
  over brand identity like colors, fonts, and how to display the menus.
- **Tenant's POS System**: `tenant.pos.foodbasket.app`. Provided as a separate
  system than the landing page to optimize for PWA-like architecture, as well
  as isolation between a work-user and a normal user.
- **The Platform-wide Admin System**: `admin.foodbasket.app`. Allows managing
  all tenants.

Other fluffs such as CDN, S3 service are out of scope for this document and will
not be detailed.

> From version 2, tenant's POS URL has been changed from `pos.tenant.foodbasket.app`
> to `tenant.pos.foodbasket.app`, to allow better and more flexible management
> with wildcard domains like `*.pos.foodbasket.app`.
>
> The rationale behind this allows for more dedicated services later on, and easier
> management in the future, such as a Media Manager at \*.media.foodbasket.app,
> for example, or a \*.kds.foodbasket.app for a Kitchen Display Service, etc.

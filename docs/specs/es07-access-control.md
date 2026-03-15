---
title: ES07 - Access Control
parent: Product & Engineering Specifications
---

# ES07 - Access Control

## Revisions

- Initial version v1 (March 4, 2026).

| Version |    Date    | Changelog                                      |
| :-----: | :--------: | ---------------------------------------------- |
|   2.1   | 2026-03-15 | Updates the pipeline to not use Origin header. |
|   1.1   | 2026-03-16 |
|   1.0   | 2026-03-04 | Initial version.                               |

## Summary

In addition to [ES02 - Authentication and Authorization](./es02-authn-authz.md),
this specification adds details to how permissions are done, and what to do when
hydrating the session with a set of permissions.

## Permission Model

### Policy Statements

Permissions are calculated via policy statements:

```json
{ "actions": ["permissions:name"], "effect": "allow" }
```

Actions are in the form of namespacing by resources, such as `modifiers:read` to
allow reading modifiers. Wildcard actions are supported up to the lowest common
denominator:

- `modifiers:*` will match `modifiers:read`, `modifiers:write` or such, but won't
  match against `menus:*`.
- `menus:12:*` will match `menus:12:read`, `menus:12:update` and such, but won't
  match against `menus:13:read` or `menus:read`.
- `*` matches against everything.

### Deny-First Calculations

Denial effect ALWAYS takes priority over allow effect. If only ONE single policy
matches, and it's a denial effect but there are multiple policies that have an
allow effect, you are unauthorized.

Wildcard permissions also take priority over the granular permissions.

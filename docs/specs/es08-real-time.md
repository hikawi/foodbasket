---
title: ES08 - Real Time Updates
parent: Product & Engineering Specifications
---

# ES08 - Real Time Updates

## Revisions

- Initial version v1 (September 2, 2026).

| Version |    Date    | Changelog        |
| :-----: | :--------: | ---------------- |
|   1.0   | 2026-09-02 | Initial version. |

## Summary

This specification adds details about how real-time updates are implemented for
this platform as a whole.

## Assymetric Model

The model is assymetric with REST for updates (such as POST, UPDATE, PATCH, DELETE,
etc.) but the client is expected to subscribe to the bus with an event stream to
receive unidirectional updates.

The reasoning for this is that, this only required real-time updates, but no need
for real-time synchronization for all actions. Mutating actions can be kept to REST
for simplicity and ease of development, whereas updates can leverage SSE for real-time
yet much simpler than WebSockets.

If one of the following constraints are true, a migration to WS may be taken into
consideration:

- Updates are short, extremely frequent and requires real-time updates on all of
  them.
- Positions of cursors for editors and management screens require real-time
  synchronization for some reason.
- More situations that I don't know yet.

### Updates

Updates for creations, patches, or deletions should use REST for simplicity.

Updates that mutate that should be notified should publish an event via `ScopedEvent`.
The `EventTarget` can be chosen by the handler, that dictates which sets of streamers
can catch that event.

All events are published to Valkey as a tag, then Valkey would publish back to the
only subscriber runner, which then does one more round of passing it down all streams.

### Reads

For reads and events, subscribe to `/v1/sse` using a GET method, which should stream
down events as the table below.

A `RequestContext` is copied at the SSE connection. If role permissions are changed,
a reload is required. This is subject to change, as it's not secure but for MVP,
it was decided for simplicity.

## Events Table

| Event  | Route     | Trigger Reason                                |
| ------ | --------- | --------------------------------------------- |
| `PONG` | /sse/ping | Pings to check whether the current SSE works. |

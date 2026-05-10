## Context

The OpenLigaDB API returns dates in ISO 8601 format (e.g. `"2025-03-08T15:30:00"`). Storing them as `String` means callers must remember the format, parse them manually, and handle errors at use-site rather than at deserialization. The `chrono` crate's serde feature handles this transparently: `chrono::DateTime<Utc>` deserialises from an ISO 8601 string and re-serialises to the same format, so the Tauri JSON bridge is unaffected.

## Goals / Non-Goals

**Goals:**
- Make date semantics explicit in the type system
- Enable date arithmetic in the library and app without ad-hoc string parsing
- Preserve the serialised JSON format for the frontend

**Non-Goals:**
- Changing timezone handling beyond what `chrono` provides out of the box
- Adding timezone-aware display formatting to the frontend (separate concern)

## Decisions

**`chrono::DateTime<FixedOffset>` for local times**: The API returns local German time with offset (e.g., `+01:00`). `FixedOffset` preserves the original offset. `when` and `last_update` use this type.

**`chrono::DateTime<Utc>` for UTC times**: `when_utc` is the UTC counterpart. Using the `Utc` marker makes the semantic difference clear.

**Keep `time_zone` as `String`**: The timezone field is an IANA name string (e.g., `"Europe/Berlin"`). `chrono-tz` could parse this, but adding that dependency is a larger scope change. Keep as `String` for now.

## Risks / Trade-offs

- The API may occasionally return malformed or missing dates; `Option<>` wrappers handle the missing case. The serde deserializer will return an error on a malformed string, which is preferable to silently propagating a bad string.
- This is a breaking change to the `Match` struct's public field types.

## Open Questions

- Should `League::season` (currently `Option<String>`) also be converted to a year integer or date? Deferred to a follow-up.

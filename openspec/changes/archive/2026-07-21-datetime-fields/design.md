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

**`chrono::NaiveDateTime` for local times**: The fixture data (`data/match-72395.json`) shows `matchDateTime` and `lastUpdateDateTime` as naive local timestamps with no UTC offset (e.g. `"2025-02-09T17:30:00"`, `"2025-02-09T19:23:27.773"`) — not `+01:00`-suffixed strings as originally assumed. `chrono::DateTime<FixedOffset>` fails to deserialize these (`premature end of input`). `NaiveDateTime` matches the actual wire format; the `time_zone` field (an IANA name string) is the only source of offset information if a caller needs to interpret these as an absolute instant. `when` and `last_update` use this type.

**`chrono::DateTime<Utc>` for UTC times**: `when_utc` is the UTC counterpart and its value (`matchDateTimeUTC`) does carry a `Z` suffix, so it deserializes cleanly as `DateTime<Utc>`.

**Keep `time_zone` as `String`**: The timezone field is an IANA name string (e.g., `"Europe/Berlin"`). `chrono-tz` could parse this, but adding that dependency is a larger scope change. Keep as `String` for now.

## Risks / Trade-offs

- The API may occasionally return malformed or missing dates; `Option<>` wrappers handle the missing case. The serde deserializer will return an error on a malformed string, which is preferable to silently propagating a bad string.
- This is a breaking change to the `Match` struct's public field types.
- `NaiveDateTime` carries no offset, so `when` and `last_update` cannot be directly compared or combined with `when_utc` (a `DateTime<Utc>`) without first pairing them with `time_zone` to resolve an absolute instant. Deferred to a follow-up if callers need that.

## Open Questions

- Should `League::season` (currently `Option<String>`) also be converted to a year integer or date? Deferred to a follow-up.

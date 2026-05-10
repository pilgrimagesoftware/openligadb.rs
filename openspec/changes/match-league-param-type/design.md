## Context

The OpenLigaDB API has two different league identifiers: a string shortcut (e.g., `"bl1"`) and a numeric ID (e.g., `4741`). Most endpoints use the shortcut; `/getnextmatchbyleagueteam` and `/getlastmatchbyleagueteam` use the numeric ID. The current code names both `league`, making the distinction invisible at the call site.

## Goals / Non-Goals

**Goals:**
- Make the parameter name self-documenting
- Prevent accidental misuse by callers following the dominant `league: &str` pattern

**Non-Goals:**
- Unifying the two league identifier styles into a single type (a `LeagueRef` enum or newtype would be a larger, breaking API change — deferred)
- Changing any runtime behaviour

## Decisions

**Rename only, for now**: Renaming `league` to `league_id` is a minimal, safe change that immediately clarifies intent. A future change can introduce a `LeagueId(i32)` newtype if desired.

## Risks / Trade-offs

- This is a technically breaking API change (the parameter name affects named-argument style callers, though Rust doesn't have named arguments). In practice, callers must update any variable they pass if they used the name `league`.

## Open Questions

- Should a `LeagueId` newtype be introduced to make the distinction impossible to confuse at the type level? (Recommended future work, but out of scope here.)

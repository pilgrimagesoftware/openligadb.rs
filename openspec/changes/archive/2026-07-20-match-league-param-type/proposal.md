## Why

`Match::next_match_by_league_team` and `Match::last_match_by_league_team` accept `league: i32` (a numeric league ID), while every other method that refers to a league uses `league: &str` (a shortcut string like `"bl1"`). This inconsistency is not documented and is easy to misuse — a caller following the pattern of other methods will pass a shortcut string and silently get the wrong URL segment. The parameter should at minimum be renamed to `league_id` to signal the difference, or the two styles should be unified.

## What Changes

- Rename the `league: i32` parameter to `league_id: i32` in both `next_match_by_league_team` and `last_match_by_league_team`
- Update the doc comments for both methods to say "the numeric league ID (not the shortcut string)" and link to `League::id`
- Update the corresponding test constants and calls in `match.rs`

## Capabilities

No behaviour change — the generated URL is identical; only parameter naming changes.

## Impact

- `openligadb.rs/src/models/match.rs`: rename parameter in two method signatures and update doc comments and test code

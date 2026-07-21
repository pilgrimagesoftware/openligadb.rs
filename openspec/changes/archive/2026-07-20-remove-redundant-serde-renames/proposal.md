## Why

Several model structs carry `#[serde(rename(deserialize = "fieldName"))]` annotations on fields whose Rust name already matches the JSON key exactly. These are noise: they add visual clutter, imply the names differ when they do not, and must be kept in sync with both the field name and the JSON schema even though they add no value.

Affected locations:
- `result.rs` → `GlobalResultInfo`: `id` renamed to `"id"`, `name` renamed to `"name"`
- `goal.rs` → `Goal`: `comment` renamed to `"comment"`

## What Changes

- Remove `#[serde(rename(deserialize = "id"))]` from `GlobalResultInfo::id`
- Remove `#[serde(rename(deserialize = "name"))]` from `GlobalResultInfo::name`
- Remove `#[serde(rename(deserialize = "comment"))]` from `Goal::comment`

## Capabilities

No behaviour change — serde serialisation output is identical.

## Impact

- `openligadb.rs/src/models/result.rs`: remove two redundant rename attributes
- `openligadb.rs/src/models/goal.rs`: remove one redundant rename attribute

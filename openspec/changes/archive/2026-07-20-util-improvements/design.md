## Context

`util.rs` contains two small async helper functions used by the model modules to make HTTP requests. They were written with a few rough edges that are easy to clean up.

## Goals / Non-Goals

**Goals:**
- Remove unnecessary conversions and bounds
- Tighten visibility to match actual usage

**Non-Goals:**
- Changing the error handling strategy (covered by `lib-custom-error-type`)

## Decisions

**1. Drop `url.as_str()`**: `reqwest::get` is generic over `impl IntoUrl`. The `url::Url` type directly implements `IntoUrl`, so passing the `Url` value avoids an unnecessary intermediate string.

**2. Drop `+ 'static`**: `DeserializeOwned = for<'de> Deserialize<'de>` means the type `M` must be deserializable without borrowing from the input, but it imposes no `'static` requirement. Adding `'static` on top is noise.

**3. `pub(crate)` visibility**: The functions are in a `mod util` that is declared without `pub` in `lib.rs`. Even with `pub` on the functions, they are not accessible outside the crate. Making them `pub(crate)` documents the intent clearly.

**4. Remove bare `use reqwest;`**: Bringing the crate name into scope with `use reqwest;` and then calling `reqwest::get(...)` as a path is redundant. Remove the use statement.

## Risks / Trade-offs

- None. All changes are backward-compatible within the crate.

## Open Questions

- None.

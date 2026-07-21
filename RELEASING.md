# Releasing

Releases are driven by [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) on `develop` and [`git-cliff`](https://git-cliff.org), via three workflows in `.github/workflows/` (thin wrappers around the `rust-*` reusable workflows in [`pilgrimagesoftware/github-actions`](https://github.com/pilgrimagesoftware/github-actions)).
There's no manual version bumping or changelog editing.

Because of this, **squash-merging a feature PR into `develop` must preserve the original commit's Conventional Commits type and any `!`/`BREAKING CHANGE:` marker.**
GitHub's default squash message is the PR title, which usually drops both — `git-cliff` then can't classify the squashed commit, silently produces an empty/no-op changelog section, and under-bumps the version.
Set the squash subject explicitly, e.g. `gh pr merge <n> --squash --subject "feat!: <description>"`, or merge with a merge commit instead when the PR is a single already-well-formed commit.

## 1. Prepare the release

Trigger **Prepare Release** manually (Actions tab → Prepare Release → Run workflow). It:

1. Runs `git-cliff --bumped-version` against `develop` to compute the next version from commits since the last tag (`fix:` → patch, `feat:` → minor, a `!` or `BREAKING CHANGE:` footer → major).
2. Bumps `Cargo.toml` to that version and prepends the generated section to `CHANGELOG.md`.
3. Pushes a `release/<version>` branch and opens (or updates, if one is already open) a PR from it into `master`, labeled `release` and requesting review.

If it fails with "nothing to release," either no `feat:`/`fix:`/etc. commits have landed on `develop` since the last tag, or no tag exists yet for `git-cliff` to compute a version relative to — see the workflow's error output for which.

## 2. Review and merge

Check the generated `CHANGELOG.md` section and the version bump on the PR.
Fix anything wrong by pushing to the `release/<version>` branch directly (or re-running Prepare Release, which force-pushes and updates the same PR).
Merge into `master` once CI passes.

## 3. Tag and publish

Merging the release PR triggers **Tag Release**, which tags `master` as `v<version>`.
The tag push triggers **Release**, which:

1. Re-runs the test suite against the tagged commit.
2. Publishes to crates.io.
3. Cuts a GitHub Release with that tag's changelog section as the release notes.
4. Merges `master` back into `develop` so the version bump and changelog aren't lost on the next release cycle.

## crates.io authentication

`release.yaml` is configured for crates.io OIDC trusted publishing (`trusted-publishing: true`), the same as `fulltime-plugin-api`.
**This requires one manual, one-time step before the first release under this pipeline**: the crate must already exist on crates.io (it does — `openligadb` has been publishing since `0.0.9`), and then be configured as a [trusted publisher](https://crates.io/docs/trusted-publishing) on crates.io pointing at this repo and the `release.yaml` workflow.
Until that's done, `Release` will fail at the "Authenticate with crates.io" step; fall back to `trusted-publishing: false` with a `CRATES_API_KEY` secret in the meantime if a release can't wait.

## Manually re-running a release

If **Release** needs to be re-run for an existing tag (e.g. crates.io publish failed after the GitHub Release was already cut), trigger it manually with the `tag` input set to the existing `vX.Y.Z` tag rather than pushing a new tag.

## Migrating from the old pipeline

Versions up to `0.0.13` were published by a different, retired workflow (`actions-rs/cargo` +
`anothrNick/github-tag-action`, since removed). That workflow had a bug: it bumped and committed
`Cargo.toml` locally in the CI runner but only ever pushed the resulting *tag*, never the commit
itself back to `develop` — so `Cargo.toml` in git history was stuck at `0.0.9` long after
crates.io had moved past it. `Cargo.toml` has been hand-corrected to `0.0.13` to match what's
actually published; the new pipeline won't repeat this since `Prepare Release` opens a real PR
with the bump committed to a branch, not a fire-and-forget `git push --tags`.

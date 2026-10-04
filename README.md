<p align="center">
  <a href="../../actions"><img src="https://github.com/PeteRichardson/vrsn/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/PeteRichardson/vrsn" alt="Latest release"></a>
</p>

# vrsn

> _A dummy Rust app for practicing build stamps and releases before they go into a real tool._

vrsn prints a greeting and nothing else. Its purpose is the machinery around it: every build carries a stamp that shows which commit it came from, whether the tree was dirty, which branch it was on, and when it was built, and one command makes a release with a version bump, a tag, a changelog entry and a GitHub release. It is the pre-flight for the versioning plan of [recon](https://github.com/PeteRichardson/recon), so that problems are found here and not there. The design is recon's [build stamps and releases spec](https://github.com/PeteRichardson/recon/blob/main/docs/specs/2026-09-29-build-stamps-and-releases-design.md).

> **Status:** Experimental. A practice sandbox. Nothing here is meant to be installed or used.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Build](#build)
- [Usage](#usage)
- [The build stamp](#the-build-stamp)
- [Releasing](#releasing)
- [Development](#development)
- [Known Limitations](#known-limitations)
- [Changelog](#changelog)
- [License](#license)

---

## Prerequisites

- **Rust** 1.85 or later (edition 2024).
- **git** on the `PATH`, for the build stamp. A build with no git still works, but the stamp shows only the version and the build time.
- **For releases only:** [`cargo-release`](https://github.com/crate-ci/cargo-release), [`git-cliff`](https://git-cliff.org) and the GitHub CLI [`gh`](https://cli.github.com), authenticated.

```sh
cargo install cargo-release
brew install git-cliff gh
```

---

## Build

```sh
git clone https://github.com/PeteRichardson/vrsn.git
cd vrsn
cargo build
./target/debug/vrsn --version
```

vrsn is not published to crates.io (`publish = false`).

---

## Usage

```
$ vrsn
Hello, vrsn 0.1.0 (main)!
```

```
$ vrsn --json
{
  "greeting": "Hello",
  "greetee": "vrsn",
  "version": "0.1.0 (main)"
}
```

```
$ vrsn --help
A dummy app to practice build stamps and releases

Usage: vrsn [OPTIONS]

Options:
      --json     Print the greeting as JSON
  -h, --help     Print help
  -V, --version  Print version

vrsn 0.1.0 (main)
checkout: /Users/pete/practice/vrsn
built: 2026-10-04T16:15:23-07:00
```

---

## The build stamp

`build.rs` makes the stamp at build time. `-V` gives the short line; `--version` and `--help` add the checkout path and the build time:

```
$ vrsn --version
vrsn 0.1.0+g549295e.dirty (Fix-I1-build-stamp)
checkout: /Users/pete/practice/vrsn/.worktrees/Fix-I1-build-stamp
built: 2026-10-04T15:19:06-07:00
```

| Part | Meaning |
|---|---|
| `0.1.0` | The version in `Cargo.toml` |
| `+g549295e` | At commit `549295e`, with no `vX.Y.Z` tag before it (`g` is for git, as in `git describe`). After a tag, the number of commits since the tag comes first: `+3.g549295e` |
| `.dirty` | Tracked files have changes that are not committed. It does not say which changes |
| `(Fix-I1-build-stamp)` | The branch, or `(detached)` |
| `checkout:` | The directory the build came from |
| `built:` | Local time of the last real rebuild |

A clean build exactly on a tag shows only `vrsn 0.1.0 (main)`. A build with no git shows `vrsn 0.1.0` and the `built:` line.

**When the stamp changes.** The build script reruns only when `src/`, `build.rs`, `Cargo.toml` or `Cargo.lock` changes, or when git's `HEAD`, index, the current branch ref, `packed-refs` or `refs/tags` changes. So a commit, a checkout, a `git add` or a new tag updates the stamp, and a `cargo build` with no changes stays instant.

**Two dirty builds of the same commit have the same `-V` line.** Use the `built:` time in `--version` to tell them apart.

---

## Releasing

Releases are made with the `/release` Claude Code skill, from the main checkout:

1. **Check.** The main checkout (not a worktree), on `main`, a clean tree, the same commit as `origin/main`, and CI passed for `HEAD`. It stops and says why if not.
2. **Propose a level** from the [Conventional Commits](https://www.conventionalcommits.org) since the last tag. Before 1.0, a breaking change proposes minor, not major. You confirm or choose another level.
3. **Release.** `cargo release` bumps `Cargo.toml`, writes the `CHANGELOG.md` entry with `git-cliff`, commits `chore(release): vX.Y.Z` and makes the annotated tag `vX.Y.Z`.
4. **Push** the commit and the tag together.
5. **Publish** a GitHub release with the new changelog entry as the notes.

The configuration is in this repo: `release.toml` (cargo-release) and `cliff.toml` (git-cliff). The release notes contain only breaking changes, `feat`, `fix` and `perf` commits. `docs`, `test`, `refactor`, `style`, `chore`, `build` and `ci` commits are left out.

<!-- 🖊 TODO: /release lives in ~/.claude/skills/release (PeteRichardson/skills), not in this repo. Link it here if that repo is public. -->

---

## Development

CI (`.github/workflows/ci.yml`) runs on every push to every branch, but not on tag pushes. Run the same steps locally before you push:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

The stamp format is pure functions in `src/stamp.rs`, with unit tests. `build.rs` includes that file with `#[path]`. `tests/cli.rs` runs the binary; it does not expect a tag or a branch name, because CI uses a shallow clone with no tags and a detached `HEAD`.

Use Conventional Commit messages: they decide the proposed release level and the release notes.

---

## Known Limitations

- **The `.dirty` flag ignores untracked files**, like `git describe --dirty`.
- **An edit to a file outside `src/` and the Cargo files** (for example this README) adds `.dirty` only at the next `git add`, commit or checkout, because the build script does not watch that file.
- **A manual `git status` can cause one extra compile.** When it refreshes stale index data (after a `touch`, or a save with no change), the index becomes newer than the build. The next build compiles vrsn one more time. The dependencies are not compiled again.
- **CI builds show `+g<hash> (detached)`**, because the CI checkout has no tags and no branch.

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md), or the [releases page](../../releases).

---

## License

<!-- 🖊 TODO: Choose a license. There is no LICENSE file and no `license` field in Cargo.toml. -->

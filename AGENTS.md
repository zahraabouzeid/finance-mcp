# AGENTS.md

Rules for every contributor, human or AI agent. They apply to all tools and all agents equally.

Project decisions such as architecture, security, testing and technology choices are documented in the specs, not in this file.

## Collaboration

- Agree on a plan before writing code.
- Do not implement anything unless the maintainer explicitly asks for it.
- Keep changes small and focused. One concern per change.
- When unsure, ask. Do not guess requirements.

## Code

- No comments, unless the maintainer asks for them.
- Clean code: clear names, small functions, one responsibility per function and module.
- Modular design with established patterns.
- Idiomatic Rust.

## Versions

- Use the latest stable versions of the Rust toolchain, edition, dependencies and tools.
- Do not rely on memory or training data for versions or APIs. Check the official source first, such as crates.io, docs.rs or the project's changelog.
- Use an older version only when the latest one is incompatible with other dependencies. Document the reason in the change.

## Git

### Permissions

- Agents never commit, push, merge, rebase or create branches, tags or pull requests unless the maintainer explicitly asks for it.
- Permission applies to the single action requested. It does not carry over to later actions.
- Never force push. Never rewrite published history.
- Never commit directly to `main`. All changes go through a pull request.

### Branches

Format: `<type>/<short-description>`

- `type` is one of the commit types below.
- `short-description` is lowercase kebab-case, at most five words.
- When a branch implements a spec change, `short-description` matches the change name.

Examples: `feat/add-workspace-skeleton`, `fix/fints-tan-timeout`, `docs/agents-rules`

### Commit messages

Format follows [Conventional Commits 1.0](https://www.conventionalcommits.org/en/v1.0.0/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

- `type`: `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `build`, `ci`, `chore`.
- `scope`: the crate or area changed. Optional when the change spans the whole project.
- `subject`: imperative mood, lowercase, no period, at most 72 characters.
- `body`: optional. Explains why, not what. Wrap at 72 characters.
- `footer`: `BREAKING CHANGE: <description>` for breaking changes, also marked with `!` after the scope. References such as `Refs: #12`.
- One logical change per commit.
- Agents add a `Co-Authored-By:` trailer only when the maintainer asks for it.

Pull requests are squash merged. The pull request title follows the commit subject format.

## Writing

Applies to docs, specs, commit messages and pull requests.

- Write in English.
- Use clean, clear and short sentences.
- Never use em dashes.

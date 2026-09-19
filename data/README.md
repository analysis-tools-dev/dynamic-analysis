This folder contains the source data for the list.

- `tools/` - one YAML file per tool
- `tags.yml` - all valid tags that can be used in tool entries
- `api/` - generated JSON output (do not edit manually)

To add a tool, create a file in `tools/` following the format described in [CONTRIBUTING.md](../CONTRIBUTING.md).

The Rust workspace in [`ci/`](../ci/) contains the renderer and pull request checker. Run `make render` from the repository root to regenerate the README and JSON API, or `make render-skip-deprecated` to reuse existing deprecation data without GitHub API requests.

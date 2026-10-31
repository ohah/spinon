# RSPress docs build result

- Command: `mise exec -- bun run docs:build`
- Result: failed with exit code 1 because the repository reported 1,131 documents with dead links.
- The line-height-specific relative links found in the new precomparison and contract entries were changed to valid local assets or GitHub links before this final run. No line-height plan, scene fixture, or runtime evidence link appeared in the remaining dead-link report.
- `spec/STATUS.md` and `spec/internal/0059-c11-grid-layout.md` still appear in the report because each contains other historical broken links. Those unrelated references were left unchanged.
- Full local console output is retained at `build/spinon/c114-line-height-sim-matrix/docs-build.log` and is not part of the PR evidence to avoid committing a 22,000-line repository-wide error log.

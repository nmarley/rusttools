# dq (dumb quotes)

Replaces smart quotes (`"`, `"`, `'`, `'`) with their "dumb" ASCII counterparts (`"`, `'`).

Has options for em dash and en dash replacements (enabled by default).

Primarily useful when an LLM generates a block of text with smart quotes that you didn't want.

## Usage

```bash
dq [OPTIONS] <FILES>...
```

Processes one or more files, replacing smart quotes in place.

## Options

*   `--em-dash`: Also replace em dashes (`—`) with double hyphens (`--`) (enabled by default).
*   `--en-dash`: Also replace en dashes (`–`) with single hyphens (`-`) (enabled by default).
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

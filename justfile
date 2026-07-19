# List available recipes
default:
    @just --list

# Install all workspace tools into ~/.cargo/bin
install:
    #!/usr/bin/env bash
    set -euo pipefail
    while IFS= read -r path; do
        name="$(basename "$path")"
        echo "==> installing ${name}"
        cargo install --path "$path" --force
    done < <(
        cargo metadata --no-deps --format-version 1 \
            | python3 -c 'import json,os,sys; m=json.load(sys.stdin); print("\n".join(os.path.dirname(p["manifest_path"]) for p in sorted(m["packages"], key=lambda x: x["name"])))'
    )

# Install a single tool: just install-one finddupes
install-one tool:
    cargo install --path {{tool}} --force

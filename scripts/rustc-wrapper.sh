#!/usr/bin/env bash
set -euo pipefail

# Cargo passes the real rustc path as the first wrapper argument. sccache
# cannot cache incremental rustc outputs, so send those calls straight to the
# compiler instead of paying proxy overhead for a guaranteed miss. It remains
# active for cacheable dependencies and for clean builds that explicitly set
# CARGO_INCREMENTAL=0, including CI.
for argument in "$@"; do
    if [[ "$argument" == incremental=* || "$argument" == -Cincremental=* ]]; then
        exec "$@"
    fi
done

if command -v sccache >/dev/null 2>&1; then
    exec sccache "$@"
fi

exec "$@"

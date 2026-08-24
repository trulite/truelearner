#!/bin/sh
set -eu

if command -v sccache >/dev/null 2>&1; then
    exec sccache "$@"
fi

# Authority workers and minimal environments remain usable before sccache is
# installed; the build is slower, never semantically different.
exec "$@"

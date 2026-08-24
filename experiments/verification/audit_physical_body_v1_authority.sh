#!/bin/sh
set -eu

repository=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$repository"

require_sha() {
    expected=$1
    path=$2
    actual=$(sha256sum "$path" | awk '{print $1}')
    test "$actual" = "$expected" || {
        echo "hash mismatch: $path" >&2
        exit 1
    }
}

require_sha e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d \
    truelearner/crates/core/src/lib.rs
require_sha 8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812 \
    truelearner/crates/arena-format/src/lib.rs
require_sha 592e90c54d28b6cb6cfdb970db3120ffe4c97c50adb3720627ff7f6c34f4900d \
    truelearner/Cargo.lock

actual_rust=$(find truelearner -type f -name '*.rs' | LC_ALL=C sort)
expected_rust='truelearner/crates/arena-format/src/lib.rs
truelearner/crates/core/src/lib.rs
truelearner/crates/core/src/main.rs'
test "$actual_rust" = "$expected_rust" || {
    echo "unclassified production Rust surface" >&2
    printf '%s\n' "$actual_rust" >&2
    exit 1
}

if rg -n 'unsafe[[:space:]]*\{|mmap|memmap' truelearner --glob '*.rs'; then
    echo "forbidden production memory surface" >&2
    exit 1
fi

if rg -n 'experiments/' truelearner --glob 'Cargo.toml'; then
    echo "production depends on experiment path" >&2
    exit 1
fi

cargo metadata --manifest-path truelearner/Cargo.toml --no-deps --format-version 1 \
    > /tmp/physical-body-v1-authority-metadata.json
python3 - <<'PY'
import json
from pathlib import Path

metadata = json.loads(Path('/tmp/physical-body-v1-authority-metadata.json').read_text())
names = sorted(package['name'] for package in metadata['packages'])
assert names == ['truelearner-arena-format', 'truelearner-core'], names
root = Path(metadata['workspace_root']).resolve()
assert root.name == 'truelearner', root
for package in metadata['packages']:
    manifest = Path(package['manifest_path']).resolve()
    assert 'experiments' not in manifest.parts, manifest
    for dependency in package['dependencies']:
        path = dependency.get('path')
        if path is not None:
            assert 'experiments' not in Path(path).resolve().parts, path
PY

echo PHYSICAL_BODY_V1_AUTHORITY_STATIC_GATE_PASS

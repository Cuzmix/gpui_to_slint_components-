#!/usr/bin/env bash
# Every check CI runs. Run from the repository root.
set -euo pipefail

pattern="std""-widgets"
echo "== no ${pattern} =="
if grep -r --exclude-dir=target --exclude-dir=.git "$pattern" .; then
    echo "found ${pattern}" >&2
    exit 1
fi

echo "== format =="
status=0
while IFS= read -r file; do
    if ! slint-lsp format "$file" | diff -q - "$file" >/dev/null; then
        echo "not formatted: $file" >&2
        status=1
    fi
done < <(find ui -name '*.slint' | sort)
[ "$status" -eq 0 ]

echo "== build and test =="
cargo build --all-targets
cargo test

echo "== no gpui in the dependency tree =="
if cargo tree --all-features -e normal,build,dev | grep -i gpui; then
    echo "gpui found in the dependency tree" >&2
    exit 1
fi

echo "== slint-viewer loads every component without warnings =="
out=$(mktemp -d)
root=$(pwd)
while IFS= read -r file; do
    while IFS= read -r name; do
        cat > "$out/preview.slint" <<SLINT
import { $name } from "$root/$file";
export component Preview inherits Window {
    width: 480px;
    height: 320px;
    $name { }
}
SLINT
        log=$(slint-viewer "$out/preview.slint" --screenshot "$out/shot.png" 2>&1 || true)
        if [ -n "$log" ]; then
            echo "$file ($name):" >&2
            echo "$log" >&2
            status=1
        fi
    done < <(grep '^export component' "$file" | grep -v 'inherits Window' | sed 's/^export component \([A-Za-z0-9]*\).*/\1/')
done < <(find ui -name '*.slint' | sort)
log=$(slint-viewer ui/gallery.slint --screenshot "$out/gallery.png" 2>&1 || true)
if [ -n "$log" ]; then
    echo "ui/gallery.slint:" >&2
    echo "$log" >&2
    status=1
fi
[ "$status" -eq 0 ]
echo "all checks passed"

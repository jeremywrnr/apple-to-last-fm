# Publish a new release: crates.io → git tag → GitHub push
release:
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version')
    echo "Publishing v${version}"

    cargo publish
    echo "Published to crates.io"

    git tag "v${version}"
    git push
    git push --tags
    echo "Pushed v${version} to GitHub"

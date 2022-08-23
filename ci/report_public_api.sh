#!/usr/bin/env sh

export LATEST_TAG_COMMIT=$(git ls-remote --sort='-version:refname' --tags origin | head -n 1 | awk '{print $1}')
export CURRENT_HEAD=$(git rev-parse HEAD)

# Print current version.
cargo public-api --version
# Report public API changes in comparison to the latest tag at origin.
cargo public-api \
    --rustdoc-json-toolchain +$1 \
    --diff-git-checkouts $LATEST_TAG_COMMIT $CURRENT_HEAD

# This is just a report script, don't fail a job run just because
# something went wrong.
exit 0

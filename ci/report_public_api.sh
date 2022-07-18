#!/usr/bin/env sh

export LATEST_TAG_COMMIT=$(git ls-remote --sort='-version:refname' --tags origin | head -n 1 | awk '{print $1}')
export CURRENT_HEAD=$(git rev-parse HEAD)

# Silence the `detached state` message, unimportant. 
git config advice.detachedHead false
# Report public API changes in comparison to the latest tag at origin.
cargo public-api --diff-git-checkouts $LATEST_TAG_COMMIT $CURRENT_HEAD
# Just move back to whatever checkout we were before.
git checkout --quiet @{-2}

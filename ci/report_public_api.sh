#!/usr/bin/env sh

if [ -z "$1" ]; then
    export TOOLCHAIN=""
else
    export TOOLCHAIN="+$1"
fi

export LATEST_TAG_COMMIT=$(git ls-remote --sort='-version:refname' --tags origin | head -n 1 | awk '{print $1}')
export CURRENT_HEAD=$(git rev-parse HEAD)
export CHECKOUT=$(git symbolic-ref --short HEAD)

# Print current version.
cargo public-api --version
# Report public API changes in comparison to the latest tag at origin.
cargo $TOOLCHAIN public-api --diff-git-checkouts $LATEST_TAG_COMMIT $CURRENT_HEAD

if [ $? -ne 0 ]; then
    # Move back to whatever checkout we were before.
    git checkout --quiet $CHECKOUT
fi

# This script must not fail a job run.
exit 0

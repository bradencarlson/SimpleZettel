#!/bin/bash

if [[ -z "$1" ]]; then 
        echo "No version given!"
        echo "usage: update-version.sh <version-number>"
fi

sed -i -E "s/version\(\"[a-zA-Z0-9\.]*\"\)/version(\"$1\")/" src/args.rs
sed -i -E "s/version = .*/version = \"$1\"/" Cargo.toml

# Update the version in Cargo.lock
cargo update -w

#!/bin/bash

exec < /dev/tty

V=$1

if [[ -z "$1" ]]; then 
        printf "Specify the new version number: "
        read -r V trash
fi

sed -i -E "s/version\(\"[a-zA-Z0-9\.]*\"\)/version(\"$V\")/" src/args.rs
sed -i -E "s/version = .*/version = \"$V\"/" Cargo.toml

# Update the version in Cargo.lock
cargo update -w

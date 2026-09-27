#!/bin/bash

# build.sh
# author: Braden Carlson
# builds both the target for my architecture as well as the x86_64-unknown arch,
# but only when the current branch is master.


BRANCH=$(git branch --show-current)

if [[ "$BRANCH" == "master" ]]; then 
        cargo build -r --features filetypes
        if [[ "$?" != "0" ]]; then 
                echo "Build of release failed!"
                return 1;
        fi
        cargo build -r --features filetypes --target x86_64-unknown-linux-gnu
        if [[ "$?" != "0" ]]; then 
                echo "Build of x86_64-unknown-linux-gnu release failed!"
                return 1;
        fi
fi

#!/bin/bash

echo 'DEPLOY: formatting project...'
if ! cargo fmt ; then
    echo 'DEPLOY: An error occurred.'
    exit 1
fi

echo 'DEPLOY: checking for uncommitted changes...'
if [[ -n $(git status -s) ]]; then
    echo 'DEPLOY: Please commit your changes.'
    exit 1
fi

echo 'DEPLOY: cleaning project...'
if ! cargo clean --package twilight-commander ; then
    echo 'DEPLOY: An error occurred.'
    exit 1
fi

echo 'DEPLOY: linting project...'
if ! cargo clippy -- -D warnings ; then
    exit 1
fi

echo 'DEPLOY: testing project...'
if ! cargo test ; then
    exit 1
fi

echo 'DEPLOY: pushing to master...'
git push origin master

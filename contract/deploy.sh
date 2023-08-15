#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

rm -rf neardev

# https://docs.near.org/tools/near-cli#near-dev-deploy
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/blog.wasm

echo "REACT_APP_$(cat "./neardev/dev-account.env")" > "../frontend/.env"
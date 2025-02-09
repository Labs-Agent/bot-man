#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <github-url>"
    exit 1
fi

GITHUB_URL=$1
PROJECT_NAME=$(basename $GITHUB_URL .git)

echo "Setting up Nitro devnode..."

if [ -d "nitro-devnode" ]; then
    echo "Removing existing nitro-devnode..."
    rm -rf nitro-devnode
fi

echo "Setting up foundry..."
curl -L https://foundry.paradigm.xyz | bash
foundryup

git clone https://github.com/OffchainLabs/nitro-devnode.git
cd nitro-devnode
./run-dev-node.sh & disown

sleep 10

cd ..
echo "Cloning target project..."

if [ -d "$PROJECT_NAME" ]; then
    echo "Removing existing project..."c
    rm -rf $PROJECT_NAME
fi

git clone $GITHUB_URL
cd $PROJECT_NAME

echo "Checking contract with cargo stylus..."
cargo stylus check

echo "Installing cargo stylus..."
cargo install cargo-stylus

echo "Adding wasm target..."
rustup target add wasm32-unknown-unknown

echo "Deploying contract..."
if DEPLOY_OUTPUT=$(cargo stylus deploy \
    --endpoint='http://localhost:8547' \
    --private-key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659" 2>&1); then
    
    CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep "deployed code at address:" | awk '{print $5}')
    ABI_JSON=$(echo "$DEPLOY_OUTPUT" | grep "Contract JSON ABI" | awk '{print $2}' | tr -d "'")
    
    if [ -n "$CONTRACT_ADDRESS" ] && [ -n "$ABI_JSON" ]; then
        echo "$ABI_JSON" > /tmp/contract_abi.json
        
        echo "success"
        echo "contract address: $CONTRACT_ADDRESS"
        echo "abi: /tmp/contract_abi.json"
        exit 0
    fi
fi

# If we get here, something failed
echo "Failure"
echo "contract address: null"
echo "abi: null"
exit 1

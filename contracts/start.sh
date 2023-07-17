#!/bin/sh
# start.sh

# Run Hardhat node as a background job
npx hardhat node &

# Sleep for a few seconds to ensure the hardhat node has started
sleep 5

# Run the command to deploy and setup for testing
npx hardhat --network localhost run demo.ts

# Keep container running
tail -f /dev/null

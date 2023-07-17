# Summa Smart contract

This serves as a test chain for the CLI application demo.

Before executing summa-playground, you need to initiate the test chain.

Firstly, install the required components, and run hardhat node.
```
contracts> npm install
contracts> npx hardhat node
```

Then, in a separate terminal pane, deploy and set up for testing:
```
contracts> npx hardhat --network localhost run demo.ts
```

Much of the code is derived from @alxkzmn's PR: https://github.com/summa-dev/summa-solvency/pull/99. 
Thanks to @alxkzmn

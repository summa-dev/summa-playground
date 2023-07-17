import { ethers } from "hardhat";
import hre from "hardhat";

type Deployment = {
  address: string;
};

type Deployments = {
  [network: number]: Deployment;
};

async function main() {
  const verifier = await ethers.deployContract(
    "src/SolvencyVerifier_15.sol:Verifier"
  );
  await verifier.deployed();

  const summa = await ethers.deployContract("Summa", [verifier.address]);

  await summa.deployed();

  console.log(`Summa deployed to ${summa.address}`);

  // Set account 1~3 balance to 1 Gwei, for demo
  const signers = await ethers.getSigners();
  
  for (let i = 1; i < 4; i++) {
    console.log(signers[i].address);
    await ethers.provider.send("hardhat_setBalance", [signers[i].address, "0x3b9aca00"])
  }

  const mockERC20 = await ethers.deployContract("MockERC20");
  await mockERC20.deployed();

  console.log(`MockERC20 deployed to ${mockERC20.address}`);
  await mockERC20.mint("0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC", 1642368560);

  let deploymentsJson: Deployments = {};
  const fs = require("fs");
  try {
    const deploymentsRaw = fs.readFileSync(
      "deployments.json",
      "utf8"
    );
    deploymentsJson = JSON.parse(deploymentsRaw);
    // Removing the previous deployment from the JSON file
    if (deploymentsJson[hre.network.config.chainId ?? 0])
      delete deploymentsJson[hre.network.config.chainId ?? 0];
  } catch (error) {
    console.log("No previous deployments found");
  }
  // Adding the new deployment to the previous deployments, indexed by network ID
  const newDeployment = {
    // Getting the contract address
    address: summa.address,
  };
  const deployments = {
    ...deploymentsJson,
    [hre.network.config.chainId ?? 0]: newDeployment,
  };
  const deploymentsStringified = JSON.stringify(deployments);
  //Save the contract address to a JSON file in backend src directory
  fs.writeFileSync(
    "deployments.json",
    deploymentsStringified
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

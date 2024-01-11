# CW Workspace

This is a workspace to develop CosmWasm contracts for Finschia.

## How To Use

You can generate a workspace template using a following command.

```
cargo generate --git https://github.com/Finschia/cw-workspace.git
```

You can generate a contract template using a following command in the workspace directory.

```
cargo generate \
    --git https://github.com/Finschia/cw-workspace.git contracts \
    --destination $PWD/contracts
```

## Contract Template Option

- minimal - This is an empty contracts, see [original repository](https://github.com/osmosis-labs/cw-minimal-template/tree/2c05d77b0c8fd0f44cc5c35f971263bc4b8e6419). 
- cw20 - This is a cw20-base contract, see [original repository](https://github.com/CosmWasm/cw-plus/tree/v1.1.2/contracts/cw20-base).
- cw721 - This is a cw721-base contract, see [original respository](https://github.com/CosmWasm/cw-nfts/tree/v0.18.0/contracts/cw721-base).

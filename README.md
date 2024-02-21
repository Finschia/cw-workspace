# CW Workspace

This is a workspace to develop CosmWasm contracts for Finschia.

## Requirements

- [cargo-generate](https://github.com/ashleygwilliams/cargo-generate)
- rust v1.69.0

> [!NOTE]
> If you use the [cw-gitpod](https://github.com/Finschia/cw-gitpod.git) environment, these tools are already installed.

## How To Use

Generate a workspace using a following command.

```
cargo generate --git https://github.com/Finschia/cw-workspace.git
```

Generate a contract template using a following command in the workspace directory.

```
cargo generate \
    --git https://github.com/Finschia/cw-workspace.git contracts \
    --destination $PWD/contracts
```

## How To Use (without workspace)

Generate a contract template without workspace.

```
cargo generate \
    --git https://github.com/Finschia/cw-workspace.git contracts \
    --d workspace=false
```

## Contract Option

- minimal - This is an empty contract. Please refer to [original repository](https://github.com/osmosis-labs/cw-minimal-template/tree/2c05d77b0c8fd0f44cc5c35f971263bc4b8e6419) for more information.
- cw20 - This is a cw20-base contract. Please refer to [original repository](https://github.com/CosmWasm/cw-plus/tree/v1.1.0/contracts/cw20-base) for more information.
- cw721 - This is a cw721-base contract. Please refer to [original respository](https://github.com/CosmWasm/cw-nfts/tree/v0.16.0/contracts/cw721-base) for more information.

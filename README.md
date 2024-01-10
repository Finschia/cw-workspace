# {{project-name}}

## How To Use

You can generate a workspace template using a following command.

```
cargo generate --git https://github.com/Finschia/cw-workspace.git
```

You can generate a contract template using a following command in the workspace directory.

```
cargo generate \
    --git https://github.com/Finschia/cw-workspace.git contracts \
    --destination ./contracts
```

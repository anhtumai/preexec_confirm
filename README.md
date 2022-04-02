# Confirm

```
A zsh `middleware` forcing user to confirm before executing commands with dangerous keywords.
```

## Demo

User can define an YAML config file, specifying keywords of dangerous commands
that he/she doesn't want to accidentally execute.
For example, `serverless deploy --stage prod-environment`

When the middleware detects a dangerous command,
it will force user to confirm by re-typing a random string, to avoid muscle memory.

![Demo](docs/demo.png)


Export env var `SKIP_CONFIRM=true` if you want to temporarily ignore the middleware.

![Skip-confirm-demo](docs/skip-confirm-demo.png)


## Usage

**Requirement**: This project is only applied for Zsh.

## Installation

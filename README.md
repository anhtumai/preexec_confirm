# Preexec Confirm

```
A zsh `middleware` forcing user to confirm before executing commands
with preconfigured texts/regex patterns.
```

## Demo

Users can define a YAML config file, specifying keywords or regex patterns
of dangerous commands that they don't want to execute by accident.
For example, `serverless deploy --stage prod-environment`

When the middleware detects a dangerous command,
it will force the user to confirm by re-typing a random string, to avoid muscle memory.

![Demo](docs/demo.png)

Export env var `SKIP_CONFIRM=true` if you want to temporarily ignore the middleware.

![Skip-confirm-demo](docs/skip-confirm-demo.png)

## Installation and Usage

**Requirements**:

- This project is only applied for Zsh.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is needed

### 1. Build from source

```bash
cargo install preexec_confirm

# ensure the system recognizes preexec_confirm path
which preexec_confirm
## ~/.cargo/bin/preexec_confirm
```

### 2. Create a YAML config file with the following format

```YAML
- contain: string
  description (optional): string
  regex (optional): bool
```

- contain: text/regex pattern to search in the command, depending on `regex` option
- description: explains why the rule is needed
- regex: if set to `true`, `contain` will be treated as a regex pattern

### 3. Ensure these lines exist in your `.zshrc` with correct order

```zshrc
function preexec_confirm_hook() {
    CONFIG_PATH="/home/anhtumai/.config/confirm/config.yml" #change this
    preexec_confirm $CONFIG_PATH $1
}

autoload -Uz add-zsh-hook

add-zsh-hook preexec preexec_confirm_hook
```

## Uninstallation

- Run `cargo uninstall preexec_confirm`
- Remove these configured lines above in `.zshrc`

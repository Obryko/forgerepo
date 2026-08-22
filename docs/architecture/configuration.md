# Configuration Architecture

ForgeRepo configuration is expected to live in `forge.toml`.

## Principles

- minimal required configuration;
- readable without extensive documentation;
- sensible defaults;
- semantic validation before feature execution;
- configuration parsing separated from CLI rendering.

## Example direction

```toml
[workspace]
include = ["apps/*", "packages/*"]

[boundaries]
frontend = ["shared", "ui"]
backend = ["shared"]
```

The concrete schema remains subject to the configuration Spike and ADRs.

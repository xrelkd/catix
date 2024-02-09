<h1 align="center">Catix</h1>

<p align="center">
    Nix binary cache proxy
</p>

<p align="center">
    <a href="https://github.com/xrelkd/catix/releases"><img src="https://img.shields.io/github/v/release/xrelkd/catix.svg"></a>
    <a href="https://github.com/xrelkd/catix/actions?query=workflow%3ARust"><img src="https://github.com/xrelkd/catix/workflows/Rust/badge.svg"></a>
    <a href="https://github.com/xrelkd/catix/actions?query=workflow%3ARelease"><img src="https://github.com/xrelkd/catix/workflows/Release/badge.svg"></a>
    <a href="https://github.com/xrelkd/catix/blob/master/LICENSE"><img alt="GitHub License" src="https://img.shields.io/github/license/xrelkd/catix"></a>
</p>

**[Installation](#installation) | [Usage](#usage) | [Configuration](#configuration)| [Container](#container)**

<details>
<summary>Table of contents</summary>

- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Container](#container)
- [License](#license)

</details>

## Installation

<details>
    <summary>Install the pre-built binaries</summary>

Pre-built binaries for Linux can be found on [the releases page](https://github.com/xrelkd/catix/releases/), the latest release is available [here](https://github.com/xrelkd/catix/releases/latest).

For example, to install `catix` to `~/bin`:

```bash
# Create `~/bin`.
mkdir -p ~/bin

# Change directory to `~/bin`.
cd ~/bin

# Download and extract catix to `~/bin/`.
# NOTE: replace the version with the version you want to install
export CATIX_VERSION=$(basename $(curl -s -w %{redirect_url} https://github.com/xrelkd/catix/releases/latest))

# NOTE: the architecture of your machine,
# Available values are `x86_64-unknown-linux-musl`, `x86_64-apple-darwin`, `aarch64-apple-darwin`.
export ARCH=x86_64-unknown-linux-musl
curl -s -L "https://github.com/xrelkd/catix/releases/download/${CATIX_VERSION}/catix-${CATIX_VERSION}-${ARCH}.tar.gz" | tar xzf -

# Add `~/bin` to the paths that your shell searches for executables
# this line should be added to your shells initialization file,
# e.g. `~/.bashrc` or `~/.zshrc`
export PATH="$PATH:$HOME/bin"

# Show version.
catix version
```

</details>

<details>
  <summary>Build from source</summary>

`catix` requires the following tools and packages to build:

- `rustc`
- `cargo`
- `pkg-config`
- `libgit2`

With the above tools and packages already installed, you can simply run:

```bash
git clone --branch=main https://github.com/xrelkd/catix.git
cd catix

cargo install --path catix
```

</details>

## Usage

```bash
# Show usage.
catix help

# Show version.
catix version

# Start proxy server.
catix --host 0.0.0.0 \
  --port 8080 \
  --upstream-servers https://cache.nixos.org \
  --upstream-servers http://127.0.0.1:8000 &

# Install package `hello` via `catix`
nix profile install --option substituters http://127.0.0.1:8080 'nixpkgs/nixos-unstable#hello'
```

## Configuration

The configuration file of `catix` is placed on `$XDG_CONFIG_HOME/catix/catix.toml`.

```bash
# Create directory to store configuration files.
mkdir -p $XDG_CONFIG_HOME/catix/

# Generate default configuration and place it on `$XDG_CONFIG_HOME/catix/catix.toml`.
catix default-config > $XDG_CONFIG_HOME/catix/catix.toml
```

<details>
<summary>Example of <b>$XDG_CONFIG_HOME/catix/catix.toml</b></summary>

```toml
[log]
# Emit log to systemd-journald.
emit_journald = true
# Emit log to stdout.
emit_stdout = false
# Emit log to stderr.
emit_stderr = false
# Set the log level, available values are "ERROR", "WARN", "INFO", "DEBUG", "TRACE".
level = "INFO"

[web]
# Host address of HTTP server.
host = "127.0.0.1"
# Port of HTTP server.
port = 37000

[metrics]
# Enable Prometheus metrics.
enable = true
# Host address of metrics.
host = "127.0.0.1"
# Port of metrics.
port = 37002
```

</details>

## Container

Container images are available on [GitHub Packages](https://github.com/xrelkd/catix/pkgs/container/catix).

- Run `catix` with `Docker`

```bash
docker pull ghcr.io/xrelkd/catix:latest
docker run -d ghcr.io/xrelkd/catix:latest
```

## License

Catix is licensed under the GNU General Public License version 3. See [LICENSE](./LICENSE) for more information.

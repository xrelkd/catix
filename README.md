<h1 align="center">Catix</h1>

<p align="center">
    A file downloader written in
    <a href="https://www.rust-lang.org/" target="_blank">Rust Programming Language</a>.
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

- [Features](#features)
- [Screenshots](#screenshots)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Container](#container)
- [License](#license)

</details>

## Features

- [x] Support downloading files from HTTP/HTTPs services
- [x] Support downloading files from SFTP services
- [x] Support downloading files from [MinIO](https://min.io/) services
- [x] Support parallel downloading to accelerate download speed
- [x] Support broken-point continuingly-transferring
- [x] Support daemonizing
- [x] Provide terminal user interface (TUI)

## Screenshots

- Terminal user interface (TUI)

![screenshot tui](docs/_static/screenshot-tui.png)

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
export CARACAL_VERSION=v0.3.1

# NOTE: the architecture of your machine,
# Available values are `x86_64-unknown-linux-musl`, `x86_64-apple-darwin`, `aarch64-apple-darwin`.
export ARCH=x86_64-unknown-linux-musl
curl -s -L "https://github.com/xrelkd/catix/releases/download/${CARACAL_VERSION}/catix-${CARACAL_VERSION}-${ARCH}.tar.gz" | tar xzf -

# Add `~/bin` to the paths that your shell searches for executables
# this line should be added to your shells initialization file,
# e.g. `~/.bashrc` or `~/.zshrc`
export PATH="$PATH:$HOME/bin"

# Show version.
catix version

# Show version.
catix-daemon version

# Show version.
catix-tui version
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
cargo install --path catix-daemon
cargo install --path catix-tui
```

</details>

## Usage

### Standalone mode

Run `catix` without `catix-daemon`

```bash
# Show usage.
catix help

# Download a file from HTTP server.
catix https://www.rust-lang.org/

# Download multiple files from HTTP server.
catix https://example.com/a.tar.gz https://example.com/b.zip

# Copy a file from local file system.
catix /etc/os-release

# Download a file from SFTP server.
catix sftp://my-ssh-server/etc/os-release

# Copy a file from MinIO server.
catix minio://myminio/path/to/file

# Download multiple files from different services.
catix \
    /etc/os-release \
    https://example.com/a.tar.gz \
    sftp://my-ssh-server/etc/os-release \
    minio://myminio/path/to/file

# Download multiple files from different services and put them in directory `/tmp/downloads`.
mkdir -p /tmp/downloads && \
    catix -D /tmp/downloads \
        /etc/os-release \
        sftp://my-ssh-server/etc/os-release \
        minio://myminio/path/to/file \
        https://example.com/a.tar.gz

# Specify an alternative number of connections.
catix -n 3 https://www.rust-lang.org/

# Set the connection timeout in second.
catix -T 3 https://www.rust-lang.org/

```

### Daemon mode

Run `catix-daemon` and use `catix` to interact with `catix-daemon`.

```bash
# Show usage.
catix-daemon help

# Start daemon, catix-daemon runs as a daemon.
# It provides gRPC endpoint, waiting for new commands.
catix-daemon &

# Show usage of `add-uri`.
catix help add-uri

# Use the subcommand `add-uri` to create new task.
# Add a new task for downloading a file from HTTP server.
catix add-uri https://www.rust-lang.org/

# Show status of tasks.
catix status

# Add a new task for downloading multiple files from HTTP server.
catix add-uri https://example.com/a.tar.gz https://example.com/b.zip

# Add a new task for copying a file from local file system.
catix add-uri /etc/os-release

# Add a new task for downloading a file from SFTP server.
catix add-uri sftp://my-ssh-server/etc/os-release

# Add a new task for copying a file from MinIO server.
catix add-uri minio://myminio/path/to/file

# Add a new task for downloading multiple files from different services.
catix add-uri \
    /etc/os-release \
    https://example.com/a.tar.gz \
    sftp://my-ssh-server/etc/os-release \
    minio://myminio/path/to/file

# Download multiple files from different services and put them in directory `/tmp/downloads`.
mkdir -p /tmp/downloads && \
    catix add-uri -D /tmp/downloads \
        /etc/os-release \
        sftp://my-ssh-server/etc/os-release \
        minio://myminio/path/to/file \
        https://example.com/a.tar.gz

# Pause tasks.
catix pause 1 2 3

# Resume tasks.
catix resume 1 2 3

# Pause all tasks.
catix pause --all

# Resume all tasks.
catix resume --all

# Remove tasks.
catix remove 1 2 3
```

### Terminal user interface (TUI)

- **NOTE**: Remember to start `catix-daemon`. Terminal user interface does not provide standalone mode, it provides user interface for user to interact with `catix-daemon`.

```bash
# Start `catix-daemon` and put it in the background.
catix-daemon &

# Show version.
catix-tui version

# Start terminal user interface.
catix-tui
```

## Configuration

The configuration file of `catix` is placed on `$XDG_CONFIG_HOME/catix/catix.toml`.

The configuration file of `catix-daemon` is placed on `$XDG_CONFIG_HOME/catix/catix-daemon.toml`.

The configuration file of `catix-tui` is placed on `$XDG_CONFIG_HOME/catix/catix-tui.toml`.

```bash
# Create directory to store configuration files.
mkdir -p $XDG_CONFIG_HOME/catix/

# Generate default configuration and place it on `$XDG_CONFIG_HOME/catix/catix.toml`.
catix default-config > $XDG_CONFIG_HOME/catix/catix.toml

# Generate default configuration and place it on `$XDG_CONFIG_HOME/catix/catix-daemon.toml`.
catix-daemon default-config > $XDG_CONFIG_HOME/catix/catix-daemon.toml

# Generate default configuration and place it on `$XDG_CONFIG_HOME/catix/catix-tui.toml`.
catix-tui default-config > $XDG_CONFIG_HOME/catix/catix-tui.toml
```

<details>
<summary>Example of <b>$XDG_CONFIG_HOME/catix/catix.toml</b></summary>

**NOTE**: `~` in a file path will be resolved to `$HOME`.

```toml
# File paths to profiles, see profile file configuration
profile_files = ["/path/to/profile/file", "/path/to/profile/file2", "~/path/to/my/profile"]

[daemon]
# Endpoint of gRPC server
# Catix connect to gRPC server via local socket with file path like "/path/to/catix-daemon/grpc.sock"
# Catix connect to gRPC server via HTTP with URI like "http://www.my.server.com/"
server_endpoint = "/path/to/catix-daemon/grpc.sock"
# Access token, remove this line to disable authentication
access_token    = "my-access-token"
# File path of access token, remove this line to disable authentication
# `access_token_file_path` is preferred if both `access_token` and `access_token_file_path` are provided.
access_token_file_path = "/path/to/access-token"

[log]
# Emit log to systemd-journald
emit_journald = true
# Emit log to stdout
emit_stdout = false
# Emit log to stderr
emit_stderr = false
# Set the log level, available values are "ERROR", "WARN", "INFO", "DEBUG", "TRACE"
level = "INFO"

[downloader]
# Path of default output directory
default_output_directory = "/path/to/default/output/directory"

[downloader.http]
# The user-agent which will be passed to HTTP server
user_agent = "Catix/0.2.0"
# The number of concurrent number of HTTP connection per task
concurrent_connections = 5
```

</details>

<details>
<summary>Example of <b>$XDG_CONFIG_HOME/catix/catix-daemon.toml</b></summary>

```toml
# File paths to profiles, see profile file configuration
profile_files = ["/path/to/profile/file", "/path/to/profile/file2"]

[log]
# Emit log to systemd-journald
emit_journald = true
# Emit log to stdout
emit_stdout = false
# Emit log to stderr
emit_stderr = false
# Set the log level, available values are "ERROR", "WARN", "INFO", "DEBUG", "TRACE"
level = "INFO"

[task_scheduler]
# The number of tasks to execute concurrently
concurrent_number = 10

[downloader.http]
# The user-agent which will be passed to HTTP server
user_agent = "Catix/0.2.0"
# The number of concurrent number of HTTP connection per task
concurrent_connections = 5

[grpc]
# Provide gRPC via HTTP
enable_http = true
# Host address of gRPC, ignored while `enable_http` is `false`
host = "127.0.0.1"
# Port of gRPC server, ignored while `enable_http` is `false`
port = 37000
# Provide gRPC service via local socket (UNIX domain socket)
enable_local_socket = true
# Path of local socket
local_socket = "/path/to/catix-daemon/grpc.sock"
# Access token, remove this line to disable authentication
access_token    = "my-access-token"
# File path of access token, remove this line to disable authentication
# `access_token_file_path` is preferred if both `access_token` and `access_token_file_path` are provided.
access_token_file_path = "/path/to/access-token"

[metrics]
# Enable Prometheus metrics
enable = true
# Host address of metrics
host = "127.0.0.1"
# Port of metrics
port = 37002
```

</details>

<details>
<summary>Example of <b>$XDG_CONFIG_HOME/catix/catix-tui.toml</b></summary>

**NOTE**: `~` in a file path will be resolved to `$HOME`.

```toml
[daemon]
# Endpoint of gRPC server
# Catix connect to gRPC server via local socket with file path like "/path/to/catix-daemon/grpc.sock"
# Catix connect to gRPC server via HTTP with URI like "http://www.my.server.com/"
server_endpoint = "/path/to/catix-daemon/grpc.sock"
# Access token, remove this line to disable authentication
access_token    = "my-access-token"
# File path of access token, remove this line to disable authentication
# `access_token_file_path` is preferred if both `access_token` and `access_token_file_path` are provided.
access_token_file_path = "/path/to/access-token"

[log]
# Emit log to systemd-journald
emit_journald = true
# Emit log to stdout
emit_stdout = false
# Emit log to stderr
emit_stderr = false
# Set the log level, available values are "ERROR", "WARN", "INFO", "DEBUG", "TRACE"
level = "INFO"
```

</details>

<details>
<summary>Example of <b>Profile</b> file</summary>

```toml
[[profiles]]
[profiles.MinIO]
# Name of profile
name         = "my-minio"
# Endpoint of MinIO server
endpoint_url = "https://my.minio.server.com"
# Access key of MinIO server
access_key   = "access_key"
# Secret key of MinIO server
secret_key   = "secret_key"

[[profiles]]
[profiles.MinIO]
name         = "my-minio2"
endpoint_url = "https://my.minio2.server.com"
access_key   = "access_key"
secret_key   = "secret_key"

[[profiles]]
[profiles.SSH]
# Name of profile
name          = "my-ssh-server"
# SSH host to connect
# It may be specified as either [user@]hostname or a URI of the form ssh://[user@]hostname[:port].
endpoint      = "my-ssh-server"
# Set the SSH user
user          = "user"
# Set the key file to use
identity_file = "/path/to/ssh/key"

[[profiles]]
[profiles.SSH]
name          = "my-ssh-server2"
endpoint      = "my-ssh-server2"
user          = "user"
identity_file = "/path/to/ssh/key2"
```

</details>

## Container

Container images are available on [GitHub Packages](https://github.com/xrelkd/catix/pkgs/container/catix).

- Run `catix-daemon` with `Docker`

```bash
docker pull ghcr.io/xrelkd/catix:latest
docker run -d ghcr.io/xrelkd/catix:latest
```

<details>
<summary>Run with <b>Docker Compose</b></summary>

We use `Docker Compose` to configurate `catix-daemon` service.

1. Create `docker-compose.yaml` and `catix-daemon.toml` with the following contents.

- `docker-compose.yaml`

```yaml
services:
  catix:
    image: ghcr.io/xrelkd/catix:latest
    ports:
      - "127.0.0.1:37000:37000"
      - "127.0.0.1:37002:37002"
    volumes:
      - ${PWD}/catix-daemon.toml:/etc/catix/catix-daemon.toml
      - downloads:/downloads
    entrypoint: ["catix-daemon", "--config=/etc/catix/catix-daemon.toml"]

volumes:
  downloads:
```

- `catix-daemon.toml`

```toml
profile_files = []

[log]
# systemd-journald is not available in container, disable it
emit_journald = false
# Emit log message to stdout.
emit_stdout   = true
emit_stderr   = false
level         = "INFO"

[task_scheduler]
concurrent_number = 10

[downloader]
# Set default output directory to `/downloads`.
default_output_directory = "/downloads"

[downloader.http]
user_agent             = "Catix/0.2.0"
concurrent_connections = 5

[grpc]
enable_http         = true
# Disable local socket because we only interact with the daemon via HTTP.
enable_local_socket = false
host                = "0.0.0.0"
port                = 37000

[metrics]
enable = true
host   = "0.0.0.0"
port   = 37002
```

**NOTE**: In order to connect the `catix-daemon` in container, `daemon.server_endpoint` in `catix.toml` should be set as `http://127.0.0.1:37000`.

2. Run `docker compose up` to start the container.
3. Run `catix add-uri https://www.rust-lang.org/` to create a new task, the downloaded file is placed on `/downloads` in the container.
4. Run `catix status` to display the status of tasks.
5. Run `docker compose down` to stop the container.

</details>

## Kubernetes

<details>
<summary>Deploy on <b>Kubernetes</b></summary>

Save the following contents to `catix.yaml` and execute `kubectl apply -f catix.yaml` to deploy `catix-daemon` on Kubernetes cluster:

```yaml
# https://kubernetes.io/docs/concepts/configuration/configmap/
kind: ConfigMap
apiVersion: v1
metadata:
  name: catix

data:
  catix-daemon.toml: |
    profile_files = []

    [log]
    emit_journald = false
    emit_stdout = true
    emit_stderr = false
    level = "INFO"

    [task_scheduler]
    concurrent_number = 10

    [downloader]
    default_output_directory = "/tmp"

    [downloader.http]
    user_agent = "Catix/0.2.0"
    concurrent_connections = 5

    [grpc]
    enable_http = true
    enable_local_socket = false
    host = "0.0.0.0"
    port = 37000

    [metrics]
    enable = true
    host = "0.0.0.0"
    port = 37002

---
# https://kubernetes.io/docs/concepts/workloads/controllers/deployment/
apiVersion: apps/v1
kind: Deployment
metadata:
  name: catix
  labels:
    app.kubernetes.io/name: catix
spec:
  selector:
    matchLabels:
      app: catix
  replicas: 1
  template:
    metadata:
      labels:
        app: catix
    spec:
      restartPolicy: Always
      volumes:
        - name: config
          configMap:
            name: catix
            items:
              - key: catix-daemon.toml
                path: catix-daemon.toml
      containers:
        - name: catix
          image: ghcr.io/xrelkd/catix:latest
          imagePullPolicy: IfNotPresent
          command:
            - "catix-daemon"
            - "--config=/etc/catix/catix-daemon.toml"
          volumeMounts:
            - name: config
              mountPath: /etc/catix/
          ports:
            - containerPort: 37000
              name: grpc
            - containerPort: 37002
              name: metrics
---
# https://kubernetes.io/docs/concepts/services-networking/service/
apiVersion: v1
kind: Service
metadata:
  name: catix

spec:
  selector:
    app: catix
  type: ClusterIP
  ports:
    - name: grpc
      protocol: TCP
      port: 37000
      targetPort: 37000
---
```

</details>

## License

Catix is licensed under the GNU General Public License version 3. See [LICENSE](./LICENSE) for more information.

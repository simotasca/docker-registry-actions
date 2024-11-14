# Docker Registry Actions

Docker Registry Actions is a service that monitors Docker registry images and automatically pulls and restarts containers from a specified Docker Compose configuration when the images are updated.

## Usage

Run `docker-registry-actions -h` to see all available configuration options.

to get logs run: `sudo journalctl -f -u docker-registry-actions.service`

## Installation

### Ubuntu

To install Docker Registry Actions on Ubuntu and set it up as a `systemd` service, use the following command:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/master/installers/linux/install.sh)
```

To stop the service and uninstall the program:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/master/installers/linux/uninstall.sh)
```

### Building from Source

Simply: `cargo build --release`

## Configuration

By default the configuration must be placed in `/etc/docker-registry-actions/config.yml`

```yaml
server:
  host: 0.0.0.0
  port: 4463
  auth_token: secret-token-for-authentication

listeners:
  demo:
    compose_path: /home/simo/dev/prove/registry-listener/compose.demo.yml # required and must exist
    watch_services:
      - demo
```

### server

http server configuration (optional)

- `server.host` (optional, default=0.0.0.0): http server host
- `server.port` (optional, default=4463): http server port
- `server.auth_token` (optional): authentication token. Authenticates the requests via `Authentication: Bearer <token>` header.

Must match the [registry endpoints configuration](https://distribution.github.io/distribution/about/configuration/#endpoints).

### listeners

each listener has 2 properties:
- `compose_path` (required): an existing docker compose configuration file. if the file doesnt exist the program will crash.
- `watch_services`: a list of valid services whose images are stored on the registry. if the services do not specify an image property the pprogram will crash.
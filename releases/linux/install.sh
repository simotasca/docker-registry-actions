#!/bin/bash

# Variables (Replace these with your details)
BINARY_NAME="docker-registry-actions"
GITHUB_URL="https://github.com/simotasca/docker-registry-actions/releases/$BINARY_NAME"
SERVICE_NAME="docker-registry-actions"
CONFIG_PATH=/etc/$SERVICE_NAME

# Download and install the binary
sudo wget -O /usr/local/bin/$BINARY_NAME $GITHUB_URL
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Create a basic configuration file
sudo bash -c "cat <<EOT > $CONFIG_PATH
server:
  host: 0.0.0.0
  port: 8080

listeners:
  # configure here your push listeners
  # es.
  # demo:
  #   compose_path: /path/to/compose.yml
  #   watch_services:
  #     - service-name-1
  #     - service-name-2
EOT"

# Create a systemd service file
sudo bash -c "cat <<EOT > /etc/systemd/system/$SERVICE_NAME.service
[Unit]
Description=A service that monitors Docker registry images and automatically restarts containers
After=network.target

[Service]
ExecStart=/usr/local/bin/$BINARY_NAME -c $CONFIG_PATH
Restart=on-failure
User=$(whoami)

[Install]
WantedBy=multi-user.target
EOT"

# Reload systemd, enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable $SERVICE_NAME
sudo systemctl start $SERVICE_NAME

# Confirm the service is running
sudo systemctl status $SERVICE_NAME

echo "Modify the configuration by editing /etc/systemd/system/$SERVICE_NAME.service"
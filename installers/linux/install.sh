#!/bin/bash

# REFERENCE:
# https://abhinand05.medium.com/run-any-executable-as-systemd-service-in-linux-21298674f66f

# Variables (Replace these with your details)
BINARY_NAME="docker-registry-actions"
BINARY_URL="https://raw.githubusercontent.com/simotasca/docker-registry-actions/refs/heads/master/installers/linux/docker-registry-actions"
SERVICE_NAME="docker-registry-actions"
CONFIG_DIR="/etc/$SERVICE_NAME"
CONFIG_FILE="$CONFIG_DIR/config.yml"


# Download and install the binary
sudo wget -O /usr/local/bin/$BINARY_NAME $BINARY_URL
sudo chmod +x /usr/local/bin/$BINARY_NAME


# Create a basic configuration file
sudo mkdir $CONFIG_DIR
sudo bash -c "cat <<EOT > $CONFIG_FILE
server:
  host: 0.0.0.0
  port: 4463

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
ExecStart=/usr/local/bin/$BINARY_NAME -c $CONFIG_FILE
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

echo "Modify the configuration by editing $CONFIG_FILE"
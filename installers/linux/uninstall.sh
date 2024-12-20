#!/bin/bash

# Variables
BINARY_NAME="docker-registry-actions"
SERVICE_NAME="docker-registry-actions"
CONFIG_PATH=/etc/$SERVICE_NAME

# Stop and disable the service
sudo systemctl stop $SERVICE_NAME
sudo systemctl disable $SERVICE_NAME

# Remove the configuration file
sudo rm -rf $CONFIG_PATH

# Remove the systemd service file
sudo rm /etc/systemd/system/$SERVICE_NAME.service

# Reload systemd to apply changes
sudo systemctl daemon-reload

# Remove the binary
sudo rm /usr/local/bin/$BINARY_NAME

echo "Service $SERVICE_NAME uninstalled and binary $BINARY_NAME removed."
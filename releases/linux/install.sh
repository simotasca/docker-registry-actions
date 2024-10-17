#!/bin/bash

# Variables (Replace these with your details)
BINARY_NAME="docker-registry-actions"
GITHUB_URL="https://github.com/simotasca/docker-registry-actions/releases/$BINARY_NAME"
SERVICE_NAME="docker-registry-actions"

# Download and install the binary
sudo wget -O /usr/local/bin/$BINARY_NAME $GITHUB_URL
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Create a systemd service file
sudo bash -c "cat <<EOT > /etc/systemd/system/$SERVICE_NAME.service
[Unit]
Description=Riavvia servizi docker compose quando vengono aggiornate le immagini sul loro registro
After=network.target

[Service]
ExecStart=/usr/local/bin/$BINARY_NAME
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
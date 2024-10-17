#!/bin/bash

# Variables (Replace these with your details)
BINARY_NAME="docker-registry-actions"
GITHUB_URL="https://github.com/simotasca/docker-registry-actions/releases/$BINARY_NAME"
SERVICE_NAME="docker-registry-actions"

# Prompt user for arguments
read -p "Enter path to compose file: " COMPOSE_PATH
read -p "Enter service name list space separated (e.g., db backend website): " SERVICE_NAME_ARG
read -p "Enter host (default: 0.0.0.0): " HOST
HOST=${HOST:-0.0.0.0}
read -p "Enter port (default: 4483): " PORT
PORT=${PORT:-4483}
read -p "Enter token secret: " TOKEN_SECRET

# Download and install the binary
sudo wget -O /usr/local/bin/$BINARY_NAME $GITHUB_URL
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Create a systemd service file
sudo bash -c "cat <<EOT > /etc/systemd/system/$SERVICE_NAME.service
[Unit]
Description=Riavvia servizi docker compose quando vengono aggiornate le immagini sul loro registro
After=network.target

[Service]
ExecStart=/usr/local/bin/$BINARY_NAME -c $COMPOSE_PATH $SERVICE_NAME_ARG --host $HOST --port $PORT -t $TOKEN_SECRET
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
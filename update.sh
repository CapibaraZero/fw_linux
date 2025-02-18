#!/bin/sh

VERSION="0.1"

echo "Stopping capibarazero API"
sudo systemctl stop capibarazero-api

echo "Cleaning up old capibaraZero release"

sudo rm -rf /var/www/html/*
sudo rm -rf /home/capibarazero/capibarazero_api

echo "Downloading capibarazero Client+API..."

# Use local URL for test
sudo wget https://github.com/CapibaraZero/fw_linux/releases/download/$VERSION/capibarazero_api  -O /home/capibarazero/capibarazero_api

sudo chmod +x /home/capibarazero/capibarazero_api 

sudo wget https://github.com/CapibaraZero/fw_linux/releases/download/$VERSION/client.zip -O /var/www/html/client.zip

sudo unzip /var/www/html/client.zip -d /var/www/html/

echo "Setting right permissions to new binary..."

sudo chown capibarazero:capibarazero /home/capibarazero/capibarazero_api
sudo setcap cap_net_raw,cap_net_admin+eip /home/capibarazero/capibarazero_api

sudo systemctl restart capibarazero-api
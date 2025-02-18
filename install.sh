#!/bin/sh

VERSION="0.1"

echo "Installing necessary packages..."

# Remove useless package that waste resources
sudo apt purge avahi-daemon -y

sudo apt update
sudo apt install arp-scan aircrack-ng yersinia git build-essential docker.io lighttpd mfoc jc mdk3 reaver lirc libnfc-bin libnfc-dev libnfc-examples libnfc-pn53x-examples libnfc6 libcjson-dev libusb-dev mfcuk libmicrohttpd-dev -y

sudo apt autoremove -y

echo 'device.name = "PN532 over I2C"
device.connstring = "pn532_i2c:/dev/i2c-1"' | sudo tee -a /etc/nfc/libnfc.conf

echo "Creating capibarazero user for API"
sudo useradd -m capibarazero

sudo mkdir /home/capibarazero/wifi_captures 

echo "Tweaking capibarazero user"
sudo usermod -aG docker capibarazero
sudo usermod -aG video capibarazero
sudo usermod -aG i2c capibarazero
sudo usermod -aG netdev capibarazero

echo "Downloading bettercap docker..."
sudo docker pull bettercap/bettercap

echo "Installing nodogsplash..."
cd /tmp

git clone https://github.com/nodogsplash/nodogsplash.git
cd nodogsplash
make
sudo make install

sudo groupadd nodogsplash
sudo usermod -aG nodogsplash capibarazero
sudo chown root:nodogsplash -R /etc/nodogsplash/
sudo chmod 777 -R /etc/nodogsplash/

sudo touch /home/capibarazero/log.txt

sudo wget https://github.com/CapibaraZero/fw_linux/raw/refs/heads/main/nodogsplash.conf -O /etc/nodogsplash/nodogsplash.conf

sudo wget https://github.com/CapibaraZero/fw_linux/raw/refs/heads/main/auth.sh -O /etc/nodogsplash/auth.sh

sudo chmod +x /etc/nodogsplash/auth.sh

echo "Downloading NFC keys assets..."

sudo wget "https://github.com/DarkFlippers/unleashed-firmware/blob/dev/applications/main/nfc/resources/nfc/assets/mf_classic_dict.nfc" -O /home/capibarazero/mf_classic_dict.nfc

echo "Patching libnfc6"

# Orrible workaround to linker issue between build and Raspberry PI OS

sudo ln -s /usr/lib/aarch64-linux-gnu/libnfc.so /usr/lib/aarch64-linux-gnu/libnfc.so.5

echo "Downloading capibarazero Client+API..."

sudo wget https://github.com/CapibaraZero/fw_linux/releases/download/$VERSION/capibarazero_api -O /home/capibarazero/capibarazero_api

sudo chmod +x /home/capibarazero/capibarazero_api 

sudo wget https://github.com/CapibaraZero/fw_linux/releases/download/$VERSION/client.zip -O /var/www/html/client.zip

sudo unzip /var/www/html/client.zip -d /var/www/html/

sudo wget https://github.com/CapibaraZero/Google_Login_Clone/releases/download/1.0/google_login_page.zip -O /home/capibarazero/google_login_page.zip

echo "Setting right permissions to capibarazero user..."

sudo chown capibarazero:capibarazero -R /home/capibarazero/

# After downloading capibara
sudo setcap cap_net_raw,cap_net_admin+eip /home/capibarazero/capibarazero_api
sudo setcap cap_net_raw+p /sbin/arp-scan

# airmon-ng can't start without root permission so limit sudo to only execute airmon-ng and nmcli
echo "%capibarazero ALL = /usr/bin/yersinia *
%capibarazero ALL = /usr/bin/pkill -9 yersinia
%capibarazero ALL = /usr/sbin/mdk3 *
%capibarazero ALL = /usr/bin/pkill -9 mdk3
%capibarazero ALL = /usr/bin/nodogsplash
%capibarazero ALL = /usr/bin/pkill -9 nodogsplash
%capibarazero ALL = /usr/bin/nmcli *
%capibarazero ALL = /usr/sbin/airmon-ng *
%capibarazero ALL = /usr/bin/ip * 
%capibarazero ALL = NOPASSWD : ALL" | sudo tee -a /etc/sudoers.d/capibarazero_sudo

echo "auth       sufficient   pam_permit.so" | sudo tee -a /etc/pam.d/sudo

echo "Creating systemd service..."

echo '[Unit]
Description=Run capibarazero API server
DefaultDependencies=no
After=network.target

[Service]
Type=simple
Environment="PROCESS_USER=capibarazero"
Environment="IR_TRANSMITTER=/dev/lirc0"
Environment="IR_RECEIVER=/dev/lirc1"
User=capibarazero
Group=capibarazero
ExecStart=/home/capibarazero/capibarazero_api
TimeoutStartSec=0
RemainAfterExit=yes

[Install]
WantedBy=default.target
' | sudo tee -a /etc/systemd/system/capibarazero-api.service

sudo systemctl daemon-reload
sudo systemctl enable capibarazero-api

echo "Installation has been completed! Please restart your Raspberry pi to complete installation"
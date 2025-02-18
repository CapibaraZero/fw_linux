dpkg --add-architecture arm64
apt install curl build-essential gcc-aarch64-linux-gnu pkg-config libdbus-1-dev:arm64 libc-dev:arm64 libnfc-dev:arm64 libusb-dev:arm64 libcjson-dev:arm64

# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update

sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"  

aarch64-linux-gnu-gcc src/nfc/nfc_poll.c -c -o src/nfc/libnfc_poll.o
aarch64-linux-gnu-ar rcs src/nfc/libnfc_poll.a src/nfc/libnfc_poll.o

cargo install cross --git https://github.com/cross-rs/cross
cross build --bin capibarazero_api --target aarch64-unknown-linux-gnu --release
#!/bin/sh

sudo apt-get update
sudo apt-get upgrade


sudo apt install hostapd dnsmasq

$INTERFACE="wlan0"
$NETWORK="192.168.4"
$SSID="MyDevice"

echo "interface $INTERFACE\
    static ip_address=$NETWORK.1/24\
    nohook wpa_supplicant" | sudo tee -a /etc/dhcpcd.conf

echo "interface=$INTERFACE\
dhcp-range=$NETWORK.2,$NETWORK.20,255.255.255.0,24h\
domain=wlan\
address=/gw.wlan/$NETWORK.1\
address=/#/$NETWORK.1" | sudo tee /etc/dnsmasq.conf

echo "interface=$INTERFACE\
ssid=$SSID\
hw_mode=g\
channel=7\
macaddr_acl=0\
auth_algs=1\
ignore_broadcast_ssid=0" | sudo tee /etc/hostapd/hostapd.conf

cargo build --release
sudo cp target/release/device-setup /usr/local/bin/device-setup
sudo cp target/release/device-setup-detector /usr/local/bin/device-setup-detector

sudo cp distribution/device-setup.service /etc/systemd/system/
sudo cp distribution/device-setup-detector.service /etc/systemd/system/
sudo systemctl enable device-setup-detector.service
sudo systemctl start device-setup-detector.service

echo "device-setup and it's detector are now installed"
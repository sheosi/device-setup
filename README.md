# device-setup

Device-setup is a lightweight service for configuring an embedded device through it's HotSpot.

## Rationale

Right now, configuring something as crucial as Wi-Fi for an embedded device (like a Raspberry Pi) means an 
assortment of steps and commands (terminal, ssh, raspi-config ...) which are cumbersome, since you need
another computer and to remember all the commands, or even plainly unthinkable for ready-made images for
the public or if we make something for a family member or a friend.

One of the easiest ways to take care of this is to use Raspberry Pi's embedded hotspot when it can't connect
to a Wi-Fi (or Ethernet for some models) and configure the connection throught there.

## Usage

1. `sh setup.sh` once for installing everything and seting up (Note: you will need [Rust](https://www.rust-lang.org/tools/install))
2. Reboot for changes to take fully effect.
3. Once the device can't find a Wi-Fi connection, the HotSpot will appear.
4. Connect to the HotSpot, your OS will tell you this is a "Captive Portal" and will redirect you to the setting page
5. Configure the device, once finished, the hotspot together with the web will disappear.

Note: At this current time this is the *intended* usage, bugs and unfinished features pose problems.

### What's missing

**TESTING!!!** Mostly of the detector and systemd-related actions (the service itself has been tested).

For the time being the device-setup service is only listening on 8080, on my testing I've found that there's something blocking port 80, as it can be accessed through the same computer but not from the outside.


## Compatibility

* Raspbery Pi (0, 3, 4) Raspbian

### By component

* Wi-fi/: NetworkManager
* Locale: Localectl
* Setup: Debian/Raspberry Pi(We specify "wlan0")

Other systems/distros/devices can be made compatible, PRs are welcome.

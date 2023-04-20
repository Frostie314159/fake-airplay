# Fake-AirPlay
This crate creates a fake AirPlay service, which will be discoverable by apple devices on the network.
## Usage
> ❗ To use fake-airplay make sure you have the **avahi daemon** running ❗

### Ubuntu

To start the avahi daemon on Ubuntu, you can install the `avahi-daemon` package using the following command:

```sh
sudo apt-get install avahi-daemon
```
Once installed, you can start the daemon by running:

```sh
sudo systemctl start avahi-daemon.service
```

After starting the avahi daemon, you can run fake-airplay by installing it using cargo:

```sh
cargo install fake-airplay
```

Once installed, you can run the fake-airplay command with the following syntax:

```sh
fake-airplay -a <NAME> -d <DEVICE_TYPE>
```
Replace `<NAME>` with the name you want the fake device to show and replace `<DEVICE_TYPE>` with the devicetype you want to show.

### Arch
To start the avahi daemon install the `nss-mdns` using 
```sh
sudo pacman -S nns-mdns
```
and run:
```sh
systemctl start avahi-daemon.service
```
After starting the avahi daemon, you can run fake-airplay by installing the shairport-sync package with the following command:

```sh
cargo install fake-airplay
```
Once installed, you can run the fake-airplay command with the following syntax:

```sh
fake-airplay <NAME> [<DEVICE_TYPE>]
```
Replace `<NAME>` with the name you want the fake device to show and replace `<DEVICE_TYPE>` with the devicetype you want to show.
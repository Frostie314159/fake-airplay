# Fake-AirPlay
This crate creates a fake AirPlay service, which will be discoverable by apple devices on the network.
## Usage
> ❗ To use fake-airplay make shure you have the **avahi daemon** running ❗

To start the **avahi daemon** install the `nss-mdns` package and run:
```sh
systemctl start avahi-daemon.service
```
After starting the avahi daemon you can run fake-airplay with:

```sh
fake-airplay "<NAME>" [<DEVICE_TYPE>]
```
Replace `<Name>` with the name you want the fake device to show and replace `<DEVICE_TYPE>` with the devicetype you want to show.
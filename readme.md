# nRF70 Driver

## Generating bindings from `nrf_wifi`

The bindings are generated as part of the build, see `build.rs`. The repository
uses a specific commit from [nrf_wifi](https://github.com/zephyrproject-rtos/nrf_wifi/).

The RPU firmware is retrieved from [sdk-nrfxlib](https://github.com/nrfconnect/sdk-nrfxlib).

To fetch the RPU firmware, do:

```sh
cd scripts
cargo run --bin fetch_rpu_firmware.rs -- <COMMIT> <OUTPUT_DIRECTORY>
```

## Acronyms

- `RPU` - Receiver Processor Unit
- `BAL` - Bus Abstraction Layer
- `FMAC` - Full MAC
- `UMAC` - Upper MAC
- `LMAC` - Lower MAC

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

## Receive buffers

The nRF70 operates with up to 3 receive queues with N receive buffers in each
queue. Each receive buffer has an unique identifier: the *descriptor*
identifier. The maximum size of a receive buffer is 1600 bytes,
plus 4 bytes for head room.

```c
                       PKTRAM Memory Area                      
+-------------------------------------------------------------+
|                                                             |
|   Transmit buffers                                          |
|                                                             |
|-------------------------------------------------------------|
|   ...                                                       |
|-------------------------------------------------------------|
|                                                             |
|   Receive Buffer 0       (1604 bytes)                       |
|-------------------------------------------------------------|
|   Receive Buffer 1       (1604 bytes)                       |
|-------------------------------------------------------------|
|   ...                                                       |
|-------------------------------------------------------------|
|   Receive Buffer 2*(N-1) (1604 bytes)                       |
+-------------------------------------------------------------+
```

On initialization, the RPU has to be notified about how
many receive buffers there are, and the spacing between them in its
memory.

Two steps are done:

![TODO] verify this

1. The descriptor identifier is written at the start of the receive
   buffer memory area (4 bytes). This makes it possible for the
   application to limit the number of buffers in total (to reduce
   memory usage etc.). For example, in a setup with 1 buffer
   for each queue, receive buffer 0 would have descriptor 0,
   receive buffer N would have descriptor 1 and receive buffer
   2N would have descriptor 2.
2. The address of the start of the data in the receive buffer
   (4 bytes past the start of the receive buffer since the
   four first bytes are for the descriptor) is written to the
   RPU at its receive command base address, offset by a
   stride multiplied with the descriptor identifier:
   `address = RX_COMMAND_BASE_ADDRESS + RPU_DATA_CMD_SIZE_MAX_RX * descriptor_identifier`

![NOTE]
This assumes that the RPU only uses those descriptor identifiers
that are written during this process (if only 3 are written, only 3
are used).

## Acronyms

- `RPU` - Receiver Processor Unit
- `BAL` - Bus Abstraction Layer
- `FMAC` - Full MAC
- `UMAC` - Upper MAC
- `LMAC` - Lower MAC

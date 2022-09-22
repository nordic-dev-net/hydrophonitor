# Audio Logger

CLI-tool for recording audio in a Linux environment.

## Usage

```bash

USAGE:
    audio-logger [OPTIONS] --name <NAME> <HOST>

ARGS:
    <HOST>    Host API to use [possible values: alsa, jack]

OPTIONS:
    -b, --batch-recording <SECONDS>    (optional) Will record in [SECONDS] batches
        --buffer-size <FRAMES>         Buffer size in frames
        --channels <CHANNELS>          Channels to record
    -h, --help                         Print help information
    -n, --name <NAME>                  Filename will be `[NAME]-yyyy-mm-dd-H:M:S.wav`
    -o, --output <PATH>                Path to save the file(s)
        --print-configs                Output the available devices and their configurations
        --sample-rate <SAMPLE_RATE>    Sample rate in Hz (default = 44,000Hz)

```

## Testing

Use the Makefile commands to build the project and run a simple test.

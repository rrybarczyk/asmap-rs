# asmap-rs
A tool to assist the [asmap](https://github.com/sipa/asmap) project read and parse RIS raw data from the [RIPE NCC](https://www.ripe.net/analyse/internet-measurements/routing-information-service-ris/ris-raw-data).
It may be extended to support other data sources.

The data is collected using Quagga routing software and stored in MRT format. 

## Run
```
Parse mrt formatted files and find asn bottleneck

USAGE:
    asmap-rs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    find-bottleneck		Reads and decompresses the MRT gz files, parses the AS Paths, determines the AS bottleneck, saves result
    download      		Downloads and saves the MRT formatted gz files
    help          		Prints this message or the help of the given subcommand(s)
```

### Download RIS Raw Data
```
asmap-rs-download 0.1.0
Downloads and saves the MRT formatted gz files

USAGE:
    asmap-rs download [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --out <OUT>  Directory to write MRT formatted gz files [default: dump]
    -n, --ripe_collector_number <RIPE_COLLECTOR_NUMBER>...
										 Range of specific RIS files to download [default: [00, 24]]
```

#### Download RIS Raw Data Examples

Download all files  from RIPE NCC (`rrc00-latest-bview.gz` through `rrc24-latest-bview.gz`) and saves the MRT formatted gz files to default `dump` directory.
```
cargo run --release download
```

Download `rrc03-latest-bview.gz` and `rrc14-latest-bview.gz` files from RIPE NCC and save the MRT formatted gz files to `dump-dir` directory.
Will create `dump-dir` if dne.
```
cargo run --release download -n 3,14 -o dump-dir
```

Download `rrc03-latest-bview.gz` file from RIPE NCC and save the MRT formatted gz files to default `dump` directory.
```
cargo run --release download -n 3
```

### Find ASN Bottleneck
```
asmap-rs find-bottleneck 0.1.0
Reads and decompresses the MRT gz files, parses the AS Paths, determines the AS bottleneck, saves result

USAGE:
    asmap-rs find-bottleneck [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <DIRECTORY>    Directory path of the MRT formatted gz files to find bottleneck of
    -o, --out <OUT>          Directory to write result [default: print to stdout]
```

### Find Bottleneck ASN Example
Finds bottleneck from the data located in the `dump` and prints bottleneck results to stdout.
```
$ cargo run --release find-bottleneck -d dump
```

Finds bottleneck from the data located in the `dump` directory and writes the bottleneck results to `bottleneck/bottleneck.<epoch>.txt`.
```
$ cargo run --release find-bottleneck -d dump -o bottleneck
```

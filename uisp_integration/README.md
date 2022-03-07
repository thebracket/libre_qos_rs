# UISP Integration for LibreQOS

This project is designed to assist [LibreQOS](https://github.com/rchac/LibreQoS/). It connects to UISP, and tries to build a full topology from there.

## Setup

### Install Rust

Go to [Rustup.rs](https://rustup.rs/) and install the Rust language and its handlers. You also need to install the `libssl-dev` package on Ubuntu with `sudo apt install libssl-dev`.

### Clone the repo

Clone this repo with `git clone https://github.com/thebracket/libre_qos_rs.git`.

### Running the program

1. Change directory into `libre_qos_rs/uisp_integration` (I used a workspace - which bundles projects together - because it's highly likely that I'll be creating other tools for this project).
2. Type `cargo build` at your command line. This builds the program in `Debug` mode. You can also use `cargo build --release` to compile with optimizations.
3. Copy `keys.ron.template` to `keys.ron`. Edit the file, and put your UISP key (I recommend read-only so you don't have to trust me) and base URL (everything up to /nms/) in the marked spots.
4. You can now use `cargo run` (or `cargo run --release`) to execute the program.

> You can also go into the `targets/` directory---either `release` or `debug`---and grab the executable from there to install elsewhere on your system.

## Usage

The first time you run the program, it will connect to UISP (or bail out with an error message if it didn't work). It reads your UISP topology and creates the following files:

* `network.json` - an initial network layout, based on your UISP site hierarchy. Everything will have a speed limit of 1gbps, since there's no reasonable way to determine your actual speed limits. This is in LibreQOS's preferred format.
* `Shaper.csv` - a list of all of your client endpoints, their IP addresses and speed limits. This is also in LibreQOS's preferred format.
* `Sites.csv` - a list of all of your sites found in the hierarchy, with speed limits listed. LibreQOS doesn't use this file.
* `AccessPoints.csv` - a list of all of your APs (including "-NoAP" items located where we couldn't figure out which AP to use). LibreQOS doesn't use this file.
* `Parentless.csv` - a list of clients for whom we couldn't figure out a location in the topology. You can fix these by adding data links into your UISP setup.

Take a look at these files. Don't edit `network.json` or `Shaper.csv` directly: these are intended to be automatically generated.

The *second* time you run the program, it loads the `Sites.csv` and `AccessPoints.csv` files. These are used to populate site and AP speed limits. So edit these two files to the speeds you want, and subsequent updates won't lose your work.

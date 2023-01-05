# Port Analyzer

A command-line tool for analyzing network ports and finding open ones.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Prerequisites

This project requires Rust to be installed on your machine. You can install Rust by following the instructions at <https://www.rust-lang.org/tools/install>.

## Installing

Clone the repository to your local machine:

```shell
git clone https://github.com/Aladeenb/learning-rust/tree/main/ports-analyzer
```

Navigate to the project directory:

```shell
cd network-port-analyzer
```

Build the project:

```shell
cargo build
```

To run the tests, use the following command:

```shell
cargo test
```

## Usage

> still working on it.

To scan for open ports on a specific host, use the following command:

```shell
cargo run -- host port1 port2 port3 ...
```

Replace host with the hostname or IP address of the target machine, and port1, port2, port3, etc. with the ports you want to scan.

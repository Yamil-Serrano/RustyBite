# RustyBite

A simple, lightning-fast WiFi QR code generator.

## Overview

RustyBite helps you quickly generate secure WiFi passwords and corresponding QR codes that allow users to connect to your network without typing complicated passwords. Perfect for homes, small businesses, or events where you need to share WiFi access securely.

## Features

- **Secure Password Generation**: Creates strong random passwords with custom length
- **WiFi QR Codes**: Generates scannable QR codes that automatically configure WiFi on mobile devices
- **Lightweight**: Minimal dependencies, small executable size

## Installation

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Run the code

```bash
# Clone the repository
git clone https://github.com/Yamil-Serrano/RustyBite.git

cd src
# Run the project
cargo run
```

## Usage

Follow the prompts:
1. Enter your WiFi network name (SSID)
2. Specify desired password length
3. RustyBite will generate a secure password and create a QR code image named `wifi_qr.png` in the current directory

Scan the QR code with any smartphone camera to connect to your WiFi network automatically!

## Example

```
Enter your Network Name: 
MyHomeNetwork
Password length: 
16
Wifi password: 8f$j2P#qLm7xZ!9k
QR code saved
```


## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE.md) file for details.


## Contact

If you have any questions or suggestions, feel free to reach out to me:

GitHub: [Neowizen](https://github.com/Yamil-Serrano)


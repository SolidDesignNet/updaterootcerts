# This simple app just reads hardcoded URLs to force Windows to update the certificates.

## compiling on Linux:

If cross isn't already installed, `cargo install cross`

Headless:
```
cross build --release --target=x86_64-pc-windows-gnu
```
GUI:
```
cross build --release --target=x86_64-pc-windows-gnu -F gui
```
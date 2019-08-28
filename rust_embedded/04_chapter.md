# Meet your board

## The "F3"

The STM32F3DISCOVERY (F3) is a microcontroller with:
- ARM Coretext-M4F core processor with hardware support for single precision floating point operations
- 256 KiB of "flash" memory
- 48 KiB of RAM
- Many "peripherals", timers, GPIO, SPI, USART, etc.
- "pins" that are exposed in the two lateral "headers"
- operates at around 3.3V
- accelerometer and magnetometer in one package
- gyroscrope
- 8 user LEDS shaped like a compass
- A second microcontroller: a STM32F103CBT. This microcontroller is actually part of an on-board programmer and debugger named ST-LINK and is connected to the USB port named "USB ST-LINK".
- There's a second USB port, labeled "USB USER" that is connected to the main microcontroller, the STM32F303VCT6, and can be used in applications


## The serial module

This module will be used to exchange data between the microcontroller in the F3 and your laptop

## The Bluetooth module

This module has the exact same purpose as the serial module but it sends the data over Bluetooth instead of over USB.



# LED Roulette

[Get code from](https://github.com/rust-embedded/discovery] chapter 5


> Microcontroller programs are different from standard programs in two aspects! 

```
#![no_std] and #![no_main].
```


Microcontroller code uses the core crate not std

no_main means that this program won't use the standard main interface, instead
#[entry] from `cortex-m-rt` crate will define the custom entry point.

`fn() -> ! {` means that there will be no return.

Also check the [./05_chapter/.cargo/config](.cargo/confid) file.
It changes the linking process.  This is required by `cortex-m-rt`.

## Build it

This needs to be cross compiled because we're developing on a laptio for bare metal.
We'll need a `--target` flag for `rustc` or `cargo`.

For microcontroller in the F3, there is a Cortex-M4F processor.  `rustc` knows how to cross compile
to Cortex-M architecture and provides 4 targets that cover the different processor families within Cortex-M.

- thumbv6m-none-eabi, for the Cortex-M0 and Cortex-M1 processors
- thumbv7m-none-eabi, for the Cortex-M3 processor
- thumbv7em-none-eabi, for the Cortex-M4 and Cortex-M7 processors
- thumbv7em-none-eabihf, for the Cortex-M4F and Cortex-M7F processors

For m4, we need `thumbv7em-none-eabihf` target.  But before compiling, we need a precompiled version 
of the standard library for our target.

`$ rustup target add thumbv7em-none-eabihf`

Now we're ready to compile

```
$ cd ./05_chapter/
$ cargo build --target thumbv7em-none-eabihf
```

To verify that we produced an ARM binary executable.
```
$ # equivalent to `readelf -h target/thumbv7em-none-eabihf/debug/led-roulette
$ cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- -file-headers
```

The return of that shows the machine as ARM so I think we're good

## Flash it

Flashing is the process of putting the binary on the microcontroller.

Once flashed, the led program will be the _only_ program in microcontroller memory.

We need to use openocd in a /tmp dir.

```
$ cd /tmp
$ openocd -f interface/stlink-v2-l.cfg -f target/stm32f3x.cfg
```

Worth mentioning:

> F3 actually has two microcontrollers. One of them is used as a programmer/debugger. The part of the board that's used as a programmer is called ST-LINK (that's what STMicroelectronics decided to call it). This ST-LINK is connected to the target microcontroller using a Serial Wire Debug (SWD) interface (this interface is an ARM standard so you'll run into it when dealing with other Cortex-M based microcontrollers). This SWD interface can be used to flash and debug a microcontroller. The ST-LINK is connected to the "USB ST-LINK" port and will appear as a USB device when you connect the F3 to your laptop.

> those .cfg files we are using instruct OpenOCD to look for a ST-LINK USB device (interface/stlink-v2-1.cfg) and to expect a STM32F3XX microcontroller (target/stm32f3x.cfg) to be connected to the ST-LINK.

openocd creates a gbd server
from the led-roulette dir
`$ <gdb> -q target/thumbv7em-none-eabihf/debug/led-roulette`

where gbd is `arm-none-eabi-gdb`, `gdb-multiarch` or `gdb`


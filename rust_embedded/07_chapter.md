# Registers

`Led` Api under the hood is writing special memory regions.

_register_, e.g. 0x48001018 is the address, is a special region of memory that controls 
a _peripheral_.
_peripherals_ are pieces of electronics that sit nect to the processor and provides
the processro with extra funcvtionality.  The processor can only do math and logic.

This register, 0x48001018, controls General Purpose Input/Outpit (GPIO) pins and can be used
to drive those pins _low_ or _high_.

## An aside: LEDs, digital putputs and boltage lebels.

A pin is electrical contact.

Some of those pins are connects to LEDs, Light Emitting Diodes, whill emit light
when voltage is applied with a certain polarity.

The microcontroller has conencted those LEDs with the right polarity so all we need is output 
non-zero boltate.  Those pins can only output two voltage levels: "low", 0 or "high", 3 volts.

> These "low" and "high" states map directly to the concept of digital logic. "low" is 0 or false and "high" is 1 or true. This is why this pin configuration is known as digital output.

RTFM to find out what the register does

`x` is used to examine.  
```
We'll use GDB's examine command: x.


(gdb) next
16              *(GPIOE_BSRR as *mut u32) = 1 << 9;

(gdb) x 0x48001018
0x48001018:     0x00000000

(gdb) # the next command will turn the North LED on
(gdb) next
19              *(GPIOE_BSRR as *mut u32) = 1 << 11;

(gdb) x 0x48001018
0x48001018:     0x00000000
```


One note, the code uses an `unsafe` block.  This is because it referenced a raw pointer


## (mis)Optimization

LLVM - compilers' backend / optimizer merges writes to a register because it doesn't know we're working 
with registers.  The risk is that that changes the behavior of the profram.

to avoid that we can use bolitale operations instead of read/writes
e.g. `ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);`

## 0xBAAAAAAD address

Not all peripheral memory can be accessed.

```
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
```

It's close to the GPIOE_BSRR but it is invalid: there is no register at this address.

```
$ cargo run
Breakpoint 3, main () at src/07-registers/src/main.rs:9
9           aux7::init();

(gdb) continue
Continuing.

Breakpoint 2, UserHardFault_ (ef=0x10001fc0)
    at $REGISTRY/cortex-m-rt-0.6.3/src/lib.rs:535
535         loop {
```

We tried to read memory that doesn't exist.  

the `aux7` crate depends on the `cortex-m-rt` crate which defines a default _hard fault_ handler.  That's why openocd placed a breakpoint and the debugger halted.

`(gdb) list` will provide more information.
`(gdb) print/x *ef`

returns 
```
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x48001800,
  r1: 0x48001800,
  r2: 0xb,
  r3: 0xc,
  r12: 0xd,
  lr: 0x800019f,
  pc: 0x80028d6,
  xpsr: 0x1000000
}
```

the pc, Program Counter, points to the instruction that generated the except.  to dissassemble

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile:
471     /checkout/src/libcore/ptr.rs: No such file or directory.
   0x080028ce <+0>:     sub     sp, #16
   0x080028d0 <+2>:     mov     r1, r0
   0x080028d2 <+4>:     str     r0, [sp, #8]

472     in /checkout/src/libcore/ptr.rs
   0x080028d4 <+6>:     ldr     r0, [sp, #8]
   0x080028d6 <+8>:     ldr     r0, [r0, #0]
   0x080028d8 <+10>:    str     r0, [sp, #12]
   0x080028da <+12>:    ldr     r0, [sp, #12]
   0x080028dc <+14>:    str     r1, [sp, #4]
   0x080028de <+16>:    str     r0, [sp, #0]
   0x080028e0 <+18>:    b.n     0x80028e2 <core::ptr::read_volatile+20>

473     in /checkout/src/libcore/ptr.rs
   0x080028e2 <+20>:    ldr     r0, [sp, #0]
   0x080028e4 <+22>:    add     sp, #16
   0x080028e6 <+24>:    bx      lr

End of assembler dump.
```

The ecepted was caused by ldr r0.  

## Spooky action at a distance.  

`BSRR` can control pins of Port E.  ODR can also control the value of pins.  ODR also lets you retrieve teh current outputn status of Port E,

consider this:
```
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;
        const GPIOE_ODR: u32 = 0x4800_1014;

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn on the NORTH LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn on the EAST LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the NORTH LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the EAST LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }

    loop {}
}
```
 
 outputs 

 ```
 $ # itmdump's console
(..)
ODR = 0x0000
ODR = 0x0200
ODR = 0x0a00
ODR = 0x0800
```

ODR's value changes everytime BSRR is written to.

## Type safe manipulation

ODR has this in the docs
> Bits 16:31 Reserved, must be kept at reset value

😬

Also registers have different read/write permissions.

But also, don't work with hexadecimal addresses.

`aux7::init()` has a type safe API to manipulate the registers of the GPIOE peripheral.

In this API each register block is modeled as a `struct` and each field represents a register.  Each 
field is a new type that exposes a comination of `read, write, modify`.  They don't take primitive values.

Instead the take a newtype that is contructed using a builder pattern and that prevent the modifcation of 
reserved parts of the register.

Consider this:

```
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}

```

when we run this and print gpioe we get a summary
```
(gdb) print gpioe
$1 = (stm32f30x::gpioc::RegisterBlock *) 0x48001000
```

but when we `(gdb) print *gpioe` we get the full view of the register block.

There is a System View Description file using the svd2rust tool.  SVD is cml that 
microcontroller vendors prodivude for register maps of their microcontroller.














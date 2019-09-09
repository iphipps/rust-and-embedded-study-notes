# LEDs, again

`aux7::init` initialized the peripherals.  But we actually need to initialize the peripherals

## Power

Most peripherals are default powered off.

(RCC) Reset and Clock Control peripheral can be used to power on or off ther others.

The registers that control power of other peripherals are 
AHBENR
APB1ENR
APB2ENR

We need to figure out which controls the GPIOE peripheral.  And also which value 0 or 1 powers on
the GPIOE,

It's AHBENR and we need to use modify,

## Configuration

GPIOE peripheral needs to be configured.  Led is an abstraction of all of that work.

To accomplish this, we need to figure out which pins we need (in the User Manual).
(8-15)
And to understand what the moder register does.
`rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());`
Then we need to modify moder to configure the pins as digital outputs.

```
gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

```

then we can do the gpioe.odr.write(|w| { w.odr8().set_bit();})

Check the [solution](./08_chapter/src/main.rs)

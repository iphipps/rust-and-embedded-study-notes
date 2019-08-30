# Clocks and timers

We get Led abstaction back but will need to figure out Delay,

## for loop delays

For loops are a bad way to implement timers.  Don't do it.

Also note that aux::nop exists

## One-shot timer

We can use a _hardware time_ which keeps precise track of time.  The timer is another peripheral
available to the microcontroller and can therefore be controlled with registers.

There are a bunch of timers on this microcontroller.  


We'll be using a basic timer: TIM6.

docs refer to _one pulse mode_

go off after some time.  wait again until the timer goes off.

registers we'll use

- `SR`, the status register.
- `EGR`, the event generation register.
- `CNT`, the counter register.
- `PSC`, the prescaler register.
- `ARR`, the autoreload register.

CR1.CEN = 1 // counter is enabled by user
CNT register resets its value to zero and on each tick increments by one
Once the CNT register has reached the value of the ARR register, the counter will be disabled by the hardware CR1.CEN=0 and an update event will be raised SR.UIF = 1

CNT register increases at a frequency of apb1 / (psc + 1) times per second.

check the [code](./09_chapter/src/main.rs)











































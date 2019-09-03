# Rust embedded

This book is a course on microcontroller-based embedded systems that use Rust as the teach language rather than good 'ol C/C++

This book is not for learning rust.  It is not for learning comprehensive electric circuit theory or electronics.

It does not intend to use any other boards other than the STM32F3DISCOVERY dev board.

## Background

### What is a microcontroller?
_a system_ on a chip.
laptop == processor, RAM sticks, hard drive, ethernet port etc.
microcontroller has those components built into a chip.

### What can a microcontroller do?

A lot.  _embedded systems_ control breaks of car, wash clothing, print documents, keep you warm, optimize fuel consumption.

They all operate without user intervention even if they expose a ui.  I most of their operation is done on their own.

The _control_ a process. They usually have one or more sensors and one or more actuators.

### When should I use a microcontroller?

Why not raspberry Pi or some other OS board?  Cost.  Microcontrollers are cheaper than computers.
Microcontrollers require fewer components.  The Printed Circuit Board (PCB is smaller and cheaper.
Other reason is power consumption.  If this needs to run on batteries, that makes a big difference.

_real time_ constraints.  Some processes require their controllers response quickly.
Like quadrocopters for example.  If the deadline is not met, the process could end in failure.  A general
purpose computer running general purpose OS has a bunch running in the background and cannot therefore 
guarantee that time deadlines are met.

### When should I _note_ use a microcontroller?

When heavy computations are involved.  Some microcontrollers do not have hardware support for f64 operations.

### Why use Rust and not C?

Cargo.  C doesn't have a robust package manager. Rust does.

### Why should I _not_ use Rust?

C ecosystem is more mature.  Off the shelf solutions for many problems already exist.  E.g. Real Time Operating Systems
(RTOS) are available commercially. (wiki reference)[https://en.wikipedia.org/wiki/Real-time_operating_system].  
These don't exist for rust as an example.






























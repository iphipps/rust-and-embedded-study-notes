# Module 1: Electrical circuits and electronics

## Analogue and digital electronics.

### Binary

Transistors are the building blocks of digital electronic devices.  They
can act like switches by being at lowest (saturated) and highest resistance(cut off) and therefore maximum and negligible current. two states

One transistor represents one bit of binary information: 0 or 1.

To store more complicated data than 0 or 1.  We can use groups of bits.

4 bits can store 0-15 decimal numbers
8 bits 255
10 bits can store 1,023 (or 1 KB)
16 -- 64 KB
32 -- 4 GB
64 -- 18 EB

As you can tell, the more bits the more storage, exponentially.

Binary is represented by the power of two.
e.g.

The number 37 is represented by: (3 x 10) + (7 x 1) or 3 lots of 10 and seven units.
In binary 37 would be represented as: 100101 or (1 x 32) + (0 x 16) + (0 x 8) + (1 x 4) + (0 x 2) + (1 x 1)


#### Floating Point Binary

Similar to scientific base10 notation.  Makes 

e.g. 
2.99 x 10<sup>8</sup> -- speed of light
6.02 x 10<sup>23</sup> -- Avogadro's mole
4.35 x 10<sup>17</sup> -- age of universe in seconds.

mantissa x 10<sup>exponent</sup>

this makes efficient use of memory in computation.

16 bit register for floating point
10 bits to mantissa 6 bits to exponent


### Converters

There are Analogue to Digital(ADC) and Digital to Analogue (DAC) Converters. e.g. temperate and pressure

Converters have ranges.  

ADC read analogue voltages within a range (0-5V) and produce a binary number.  The assumption is that
both ranges, analogue and digital are linear.

Sampling rate is the rate at which input voltage is read.

Quantization rate is the error when going digital to analogue.  For that you can
increase bits of the converter or increase the sampling frequency.

#### Nyquist theorem

In order to be able to reproduce analogue signal, the sample rate must be 2x faster than the signals highest frequency component.

### Electronics kits
For this course we recommend
- Arduino Uno starter kit.  
- Raspberry Pi (Pi 3 model B)
- Iduion's 37 in 1 Sensor kit for Arduino (ST1065).


### Tinkercard

allows a gui simulation of connecting controllers to resistors, capacitors, potentiometers, etc.



































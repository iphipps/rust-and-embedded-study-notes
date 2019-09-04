# Intermediary Devices

## Microcontroller unit vs microprocessor.
Microcontroller units (MCUs) are relatively simple. no operating system.
Each to interface with sensors.
Usually has sufficient processing power.
Simple so generally less vulnerable to security attacks.

Microprocessor is a small computer with an os. 


IoT hardware can  generally be categorized into two groups: end devices and gateways.
Gateways generally control communication to the cloud or handle significant processing.

## RTOS - real time operating system

Demand data may be processed without buffering delays

### RIOT Os 
Small os for networked, memory constrained system with a focus on low power wireless devices.

## Selecting a platform

### Power management

Power requirement variables:
- clock rate of CPU
- number of processing cores
- included peripherals
- processor load
- supply voltage levels

e.g. Arduino requires less power than a raspberry pi.

### Sensors and actuators

#### Sensors
Data Acquisition (DAQ)

Signal conditioning and resolution of the sensor are considerations

Is the sensor good enough?

#### Actuators

What actions are required, does our system support implementation of those actions.

### Processing Capability

Where are we processing the data?  On the edge or in the cloud?

### Connectivity

We need to connect it to the internet otherwise its not an IoT thing.  The microcontroller needs to
talk to the sensors/actuators and also connect to the internet either directly or via a gateway.

## System on a chip - SoC

Bundled capabilities:
- data processing
- storage
- networking

## Bits
can be used to describe:
data bus width
addressing width
register width

RISC Processor - reduced instruction set
breaks actions that manipulate memory and calculation actions. Moves from register to register
CISC Processor - complex instruction set
memory manipulation, storage and retrieval in the same instruction

### ARM microprocessor
RISC style architecture.  ARM licensees can implement on their chips. M0s all have common instructions from ARM.

## Do I need an os?
Security? Memory footprint? Modularity / compatibility? Support and reliability?

## Sensor Communication
Must communicate
Serial communication protocol.
Synchronous and asynchronous

### Asynchronous serial communication

Transmitter and receiver agree on data rate, voltage level and how to interpret voltage lebel.


### Synchronous serial communication
I2C or SPI are most common.  Used for simpler devices with no internal clock
crystal and few memory registers.



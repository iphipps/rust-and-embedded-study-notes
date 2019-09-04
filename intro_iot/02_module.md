# End of the line things

This chapter will talk about sensors and actuators.

## Control systems

All internet of things systems have common characteristics.

- The perceive aspects of the world.
- The produce an action in the world, they can move, turn on, or activate something.
- They have some intelligence -- that uses sensed information to make decisions about when to produce the action.

Open loop control systems are reactive systems that control the world based on current information only.

Closed loop systems make decisions based on current information AND the effect their last action had on
the world (feedback). e.g. ac in your house. you set the temperature and the condition knows
when to start and stop operation in order to keep the room template close.

## Sensor functions and uses

Lots of sensors

- Temperatures
- Joystick (x and y axis)
- Moisture
- touch
- microphone
- infrared receiver
- etc.

### Transducers

How are devices able to perceive things that are not electrical in nature: wind speed, temperature, water.
Transducers convert one form of energy to another. E.g. Quartz crystals produce voltage from mechanic shape. They're
used in oscillators.

They can change physical phenomena to voltage or resistance.

Sometimes output of transducers need to be conditioned to a voltage that is perceivable by the computer. Amplifiers e.g.

Transducers	- Materials or devices that have the property of converting one kind of energy into another.

Strain Gauge - Pressure-sensitive resistive bridge networks that are external biased in such a way as to produce an output voltage in proportion to the amount of force and/or strain being applied to the sensor (Electronics Tutorials).

Negative coefficient (and positive coefficient)	- With a negative coefficient correlation - if the analogue phenomenon increases, then the physical property decreases. With positive correlation - if the phenomenon increases, the property also increases.

Signal conditioning	- Improving the quality of the signals going to ADCs or produced by DACs.

Instrumentation amplifiers - Instrumentation amplifiers take the difference of potential between their two inputs (a very small signal), and amplify it to produce a voltage that can be fed into an ADC.

Wheatstone Bridge	- Interconnects four resistors, forming a bridge, and is used for the measurement of an unknown resistance (Circuit Digest)


*A Sensor* is the combination of a transducer and it's signal conditioner.

*An actuator* is the combination of conditioning plus the element being controlled by the computer.


### Sensors continued

Sensors convert the physical environment to digital or analogue.  There are many different sensors but their implementation of interfacing
is similar.

We can generally classify them according to their properties:
- Analog or Digital output signal
- Transduction principles -- physical and/or chemical effects used to read
- Material and technology
- property
- application


Read this [whitepaper](http://www.ni.com/en-us/innovations/white-papers/13/sensor-terminology.html) for these terms:

- Sensitivity = Minimum input of physical parameter that will create a detectable output change.
- Precision = the reproduce-ability of the result
- Range = Max and Min values that can be measured.
- Accuracy = Max difference between actual value and the value indicated at the output
- Linearity = An expression of the extent to which the actual measured curve departs from the ideal
- Response time = output state changes to new state over a period of time.

We can also generalize
1. Environmental condition: limits for temp/humidity. environment factors can have an effect on the readings.
e.g. accelerometers are sensitive to temperature.
2. Calibration: essential for most devices.
3.  Sensors can be classified on *power* or *energy supply requirement*:
  - *Active* sensor: requires power.
  - *Passive* sensor: like rfid tags, termisistors and temperature depend resisters.

Properties:
temperature
pressure
flow
level sensors
proximity / displacement
biosensors
image
gas/chemical
inertial measurement - like gyroscopes, accelerometers, magnetometers
others: moisture, humidity, tilt, force, viscosity.

### Sensor output

The output on a sensor might be 0V or 5V for on/off or 0V through 05V for spectrum like temperature.


### Wired local communication networks

### Communication Protocols

#### Letters as Binary Numbers

ASCII -- American Standard Code for Information Interchange
A - 01000001
B - 01000010
C - 01000011

etc

For sending data from one place to another, consider the following:
- the distance
- the speed it needs to move
- safety

Local is usually wired

Some concepts:
Protocol -- rules/conventions that govern the exchange of information.
Synchronous vs Asynchronous -- synch systems use a common clock.  Asynchronous systems do not.
Peer - entities that exist on the same level of a network. 
Point to point, multi-drop, multi-point -- two devices using dedicated set of wires, many devices sharing set of wires (bus). Multi-point is where one master connects to many slave using dedicated wired. Multi-point is a number
of devices connected to a bus. 
Master/Slave networks -- master can talk to the slaves. Slaves cannot talk to each other.  Slaves must wait for master to grant access.
Multi Master -- some master/slave protocols allow more than one master.  For these arbiters will decide which master will take control of the channel and when.
Full-duplex channel -- both end points can transmit information at the same time, generally over different wires.
Half-duplex channel -- only one end point can transmit information at on time, usually over a shared wire.
Maximum number of devices -- this depends on the physical properties of the cable (capacitance resistance) or limits defined by protocol.
Maximum speed -- depends on the processing speed of the end points and the physical properties of the medium (capacitance, inductance).
Maximum distance -- max length of a cable depends on its physical properties and how fast info is transmitted.

### RS-232

RS-232 is a protocol from the 1960s to connect computers with modems.  The standard defines '1' as negative voltage between -3 and -25 V and '0' as positive 
between +3 and +25,  Most PCs use -13 and +13.

#### Clock mechanism
asynchronous protocol with dedicated control lines for facilitating the exchange of info between
Data Communications Equipment (DCE)(the modem)  <==> Data Terminal Equipment (DTE)(the computer)  

#### Handshake and timing
1. RTS (request to send) is put to on by the DTE
2. DCE puts the CTA(Clear to Send) line into the ON state
3. DTE responds by placing the DTR (Data Terminal Ready) line into the ON state
4. DTR remains on while data is being transmitted.

### I<sup>2</sup>C (aka I2C or IIC) the Inter-Integrated Circuit
I<sup>2</sup>C is a popular master-slave, multi-drop communication protocol used to exchange information over short distances, typically
on the same board.  This is nice because of low number of wires:
1. Serial Data Line (SDA)
2. Serial Clock Line (SCL)


#### Clock Mechanism
Each device is a mater or a slave. The master pulses the clock for each transmitted bit and the slave follows this speed.  The slave can hold down the clock
to pause communication (clock stretching) which ensures the slave is protected from being overwhelmed.

#### Handshake exchange and timing

Each message begins with a START and a STOP.
- Single message where master writes data to a slave.
- Single message where master reads data to a slave.
- Combined messages, where a master issues two or more reads ands or writes to one of more slaves.

A HIGH to LOW transition on the SDA line while SCL is HIGH indicates a START condition. This is always initiated by the Master.

A LOW to HIGH transition on the SDA line while SCL is HIGH defines a STOP condition. See the following timing diagram.

7 bit address identify devices.  Theoretically up to 128 devices can be connected to same bus
But some are reserved so really it is 112.




### SPI - Serial Peripheral Interface
synchronous master-slave full-duplex interface is popular for short distance communication.  There is
no spec for the protocol so message sizes and data rates vary.

#### Connections

4 wires minimum
- SCLK - Serial Clock - sent by master to slaves
- MOSI - master out, slave in.
- MISO - master in, slave out.
- SS - Slave Select -- unique to each slave. As more slaves are added, more of these get added.

#### Clock Mechanism

SPI modes 
0 - starts low - data received falling edge of clock
1 - starts low - data received rising edge of clock
2 - starts high - data received falling edge of clock
3 - starts high - data received rising edge of clock

Devises then use rising or falling to receive/read and the opposite to write/send

#### Handshake exchange and timing
- SS - held high and lowered to indicate when a data line should be written to or read from
- MOSI - held high or low by master before a clock edge.
- MISO - held high or low by slave before a clock edge.
- SCLK - clock signal sent by the master to any slaves.


## Sensor Communication

Turbidity sensor detects particles in a water solution.
Sensor communication differs based on how the sensor was set up. 
But it seems as if a microcontroller can communicate with multiple using different
communication protocols.

















































































































# Hello World

First let us connect with a jumper cable.  (no soldering for now).

ITM stands for Instrumentation Trace Macrocell is a communication protocol on top of SWD Serial
Wire debug.  Used for sending messages _one way_.

OpenOCD receives these frames.  We need to parse them with `itmdump`

Now we can use `iprintln`


## Panic

panic! macro sends output to the ITM.



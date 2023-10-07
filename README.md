This is a little side project of mine, where i attempt to create my own emulated retro console. The specs for the console are currently unknown and will be decided, once the project started to take shape.

## Current ideas
### CPU
The CPU may consist of a total of 8 registers and is a 8-bit CPU.

Instruction set is currently unknown.

### RAM
Size of RAM is currently unknown.

Right now, an idea is to have a certain part used for graphics and the rest for the game itself.

### Game data
The game data should consist of 2 parts. The first part is called memory data. This part consists writable data, which can be used for saving data.
The second part is the program- or ROM-data. Here lies the actual game code which should be read-only.

### Loading data into RAM
When the game is being loaded, it first loads all the memory data into the RAM. After that, the main-function from the program data gets loaded into the RAM.

#### Functions
The ROM consists of a bunch of functions that can get called. Once a function gets called, the instructions get loaded into the RAM and there get executed. The end of a function is marked with a return instruction, that cleans up the RAM of the function to free space.

## Questions
* When you start the console, how does the CPU know, that it should first load the memory data into the RAM and then the main-function of the ROM?
    * Maybe include a MCU that handles a bunch of stuff
* When cleaning up a function, how can we make sure, that we jump back to the previous function that called the now cleaned up function?
* How to handle user input?
* Graphics
  * How to handle those?
  * How much RAM do we need for the graphics?
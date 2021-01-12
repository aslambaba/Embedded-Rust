## Send Data from Micro-controller to host using itmdump

### After Installation of itmdump

- Connect PB3 and SWO pins on STM32F3 Board using juming Wires.
- create file itm.txt on the same location where openocd run. (defult in /tmp directory)
    - cd tmp
    - touch itm.txt
- After Running gdb run two other commands: 
    - moniter tpiu config internal itm.txt uart off 8000000
    - moniter itm port 0 on


### Debug Flash Code into Micro-controller Using OpenOcd and Gdb

- Please Follow these instructions line by line.

### Build Your Code 

- cargo build --target thumb7em-none-eabhif

#### After Installation of openocd

- Change the directory into tmp directory
    - cd tmp
- Run: openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

#### After Installation of gdb

- gdb-multiarch target/thumbv7em-none-eabihf/debug/<project-name>
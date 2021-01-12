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
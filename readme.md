## Embedded Rust Programming with STM32F303 Discovery Board

#### Required Tools 

- Rust (Version 1.31 or newer)
- OpenOCD (Tested Version v0.9.0 and v0.10.0)
- GDP -> arm-none-eabi-gdb (Version 7.12 or newer and Tested Versions 7.10 7.11 7.12 and 8.1)
- itmdump (Version 0.3.1)
- Cargo-binutils (Version 0.1.4 or newer)
- Putty for Window or Minicom for Linux (Tested Version of Minicom 2.7)

#### Required Hardware 

- STM32F303 Discovery Board
- Bluetooth Module (HC05 or HC06) 
- Serial Module
- Bread Board
- Jumper Wires

#### Installations 

- Itmdump -> cargo install itm --vers 0.3.1
- Cargo-Binutils -> rustup component add llvm-tools-preview then cargo install cargo-binutils --vers 0.1.4

##### OS Specific Installations

###### For Windows

- GDB [download](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
- Putty [download](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html)
- ST-Link Driver [download](https://www.st.com/en/development-tools/stsw-link009.html)

###### For Linux 

- Optional Packages -> sudo apt-get install bluez rfkill















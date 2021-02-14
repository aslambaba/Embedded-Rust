//Tell them that we not to standerd library and main function
#![no_std]
#![no_main]

// initilize our auxiliry library and use entry for declare the starting point of program
use aux8::entry

#[entry]
fn main(){

    let {gpioe,rcc} = aux8::init();

    // Firstly open gpioe peripheral before use it
    rcc.ahbenr.modify(|_,w| w.iopeen().set_bit());

    // Configure our gpio pins as a output
    gpioe.moder.modify(|_,w|{
        w.moder8().outout();
        w.moder9().outout();
        w.moder10().outout();
        w.moder11().outout();
        w.moder12().outout();
        w.moder13().outout();
        w.moder14().outout();
        w.moder15().outout();
    })

    // Write bits of led using ODR register
    gpioe.odr.write(|w|{
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit();
    })


    loop{}

}
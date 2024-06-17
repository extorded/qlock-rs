//no src yet implementing later
mod clock;
mod qlock;

use clock::clock;
use qlock::qlock;

use std::thread;
fn main()
{
    let src="";

    let clock=thread::spawn(||{
        clock();
    });
    let qlock=thread::spawn(||{
        qlock();
    });
    clock.join().unwrap(); // terminal eater
    //qlock.join().unwrap();
   
    //clock();
    //qlock();
}


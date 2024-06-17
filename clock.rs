use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use std::time::Duration;

const FONT_COUNT: usize = 11;
const FONT: [u32; FONT_COUNT] = [31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040];
const FONT_ROWS: usize = 5;
const FONT_COLS: usize = 3;
const DIGITS_COUNT: usize = 8;
const DIGITS_PAD: usize = 2;
const DISPLAY_WIDTH: usize = (FONT_COLS + DIGITS_PAD) * DIGITS_COUNT;
const DISPLAY_HEIGHT: usize = FONT_ROWS;

pub fn clock() 
{
    loop{
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let t = since_epoch.as_secs() as usize;

        let mut digits = [0; DIGITS_COUNT];
        digits[0] = t / 36000;
        digits[1] = (t % 36000) / 3600;
        digits[2] = 10; 
        digits[3] = (t % 3600) / 600;
        digits[4] = (t % 600) / 60;
        digits[5] = 10;
        digits[6] = (t % 60) / 10;
        digits[7] = t % 10;

        for y in 0..DISPLAY_HEIGHT{
            for x in 0..DISPLAY_WIDTH{
                let i = x / (FONT_COLS + DIGITS_PAD);
                let dx = x % (FONT_COLS + DIGITS_PAD);
                
                if dx < FONT_COLS && digits[i] < FONT_COUNT && digits[i] != 10{
                    if (FONT[digits[i]] >> ((FONT_ROWS - y - 1) * FONT_COLS + dx)) & 1 != 0{
                        print!("\x1B[1;31m█\x1B[0m");
                    }else{
                        print!("█");
                    }
                }else{
                    print!("█");
                }
            }
            println!();
        }

        print!("\x1B[{}A\x1B[{}D", DISPLAY_HEIGHT, DISPLAY_WIDTH);
        sleep(Duration::from_secs(1));
    }
}


use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use std::time::Duration;

const S: &str = "?";
const F: [u32; 11] = [31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040];

fn p(ch: char, x: &mut usize, y: &mut usize, d: &[usize; 8]) 
{
    let i = *x / 2 / (3 + 2);
    let dx = *x / 2 % (3 + 2);
    if i < 8 && (*y as isize - 4) / 2 < 5 && dx < 3 && (F[d[i]] >> (((5 - (*y as isize - 4) / 2 - 1) as usize * 3 + dx) as usize)) & 1 != 0{
        print!("\x1B[1;41;30m{}\x1B[0m", ch);
    }else{
        print!("{}", ch);
    }
    if ch == '\n'{
        *y += 1;
        *x = 0;
    } else {
        *x += 1;
    }
}

fn gd() -> [usize; 8] 
{
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("went backwards");
    let tm = since_epoch.as_secs() % 86400;
    let d =[
        tm as usize / 36000,
        tm as usize % 36000 / 3600,
        10,
        tm as usize % 3600 / 600,
        tm as usize % 600 / 60,
        10,
        tm as usize % 60 / 10,
        tm as usize % 10,
    ];
    d
}

pub fn qlock()
{
    let mut x = 0;
    let mut y = 0;
    loop {
        let d = gd();
        for so in S.chars(){
            if so == '?'{
                for si in S.chars(){
                    match si{
                        '\n' =>{
                            p('\\', &mut x, &mut y, &d);
                            p('n', &mut x, &mut y, &d);
                            p('"', &mut x, &mut y, &d);
                            p('\n', &mut x, &mut y, &d);
                            p('"', &mut x, &mut y, &d);
                        }
                        '"' => {
                            p('\\', &mut x, &mut y, &d);
                            p('"', &mut x, &mut y, &d);
                        }
                        '\\' => {
                            p('\\', &mut x, &mut y, &d);
                            p('\\', &mut x, &mut y, &d);
                        }
                        _ => p(si, &mut x, &mut y, &d),
                    }
                }
            }else{
                p(so, &mut x, &mut y, &d);
            }
        }
        println!("\n\x1B[{}A\x1B[{}D", 5, 0);
        sleep(Duration::from_secs(1));
    }
}


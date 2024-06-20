use std::thread::sleep;
use std::time::{Duration, SystemTime};

const f: [u32; 11] = [31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,];

fn gt() -> libc::tm
{
    unsafe {
        let mut tm: libc::tm = std::mem::zeroed();
        let t = libc::time(std::ptr::null_mut());
        libc::localtime_r(&t, &mut tm);
        tm
    }
}

fn d(g: usize, tm: &libc::tm) -> usize
{
    let h = tm.tm_hour;
    let m = tm.tm_min;
    let q = tm.tm_sec;
    match g {
        0 => h as usize / 10,
        1 => h as usize % 10,
        2 => 10,
        3 => m as usize / 10,
        4 => m as usize % 10,
        5 => 10,
        6 => q as usize / 10,
        7 => q as usize % 10,
        _ => panic!("invalid index"),
    }
}

fn p(ch: char, x: &mut usize, y: &mut usize, d: &[usize; 8]) 
{
    let y4 = *y / 2 - 1;
    let x2 = *x / 2 - 2;
    let i = x2 / 5;
    let j = x2 % 5;
    if *x > 2 && i < 8 && y4 < 5 && j < 3 && (f[d[i]] >> (12 - y4 * 3 + j)) & 1 != 0 {
        print!("\x1B[1;41;30m{}\x1B[0m", ch);
    } else {
        print!("{}", ch);
    }
    if ch == '\n' {
        *y += 1;
        *x = 0;
    } else {
        *x += 1;
    }
}

fn main() 
{
    let mut x = 0;
    let mut y = 0;

    loop {
        let tm = gt();
        for so in "?".chars() {
            if so == '?' {
                for si in "?".chars() {
                    match si {
                        '\n' => {
                            p(
                                '\\',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                'n',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                '"',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                '\n',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                '"',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                        }
                        '"' => {
                            p(
                                '\\',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                '"',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                        }
                        '\\' => {
                            p(
                                '\\',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                            p(
                                '\\',
                                &mut x,
                                &mut y,
                                &[
                                    d(6, &tm),
                                    d(7, &tm),
                                    10,
                                    d(3, &tm),
                                    d(4, &tm),
                                    10,
                                    d(0, &tm),
                                    d(1, &tm),
                                ],
                            );
                        }
                        _ => p(
                            si,
                            &mut x,
                            &mut y,
                            &[
                                d(6, &tm),
                                d(7, &tm),
                                10,
                                d(3, &tm),
                                d(4, &tm),
                                10,
                                d(0, &tm),
                                d(1, &tm),
                            ],
                        ),
                    }
                }
            } else {
                p(
                    so,
                    &mut x,
                    &mut y,
                    &[
                        d(6, &tm),
                        d(7, &tm),
                        10,
                        d(3, &tm),
                        d(4, &tm),
                        10,
                        d(0, &tm),
                        d(1, &tm),
                    ],
                );
            }
        }
        println!("\n\x1B[{}A\x1B[{}D", 5, 0);
        sleep(Duration::from_secs(1));
    }
}

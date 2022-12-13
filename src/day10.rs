fn draw_pixel(crt: &mut [[char; 40]; 6], sprite: i32, cno: i32) {
    let row = (cno - 1) / 40;
    let col = (cno - 1) % 40;
    if (sprite - col).abs() <= 1 {
        crt[row as usize][col as usize] = '#';
    }
}

pub fn solve(input: String) {
    let mut reg = 1i32;
    let mut cycles_done = 0i32;
    let mut signal_strength = 0;
    let mut crt = [['.'; 40]; 6];
    for line in input.lines() {
        let instr = &line[..4];
        match instr {
            "addx" => {
                if (cycles_done + 1) % 40 == 20 {
                    signal_strength += (cycles_done + 1) * reg;
                } else if (cycles_done + 2) % 40 == 20 {
                    signal_strength += (cycles_done + 2) * reg;
                }
                draw_pixel(&mut crt, reg, cycles_done + 1);
                draw_pixel(&mut crt, reg, cycles_done + 2);
                cycles_done += 2;
                reg += line[5..].parse::<i32>().unwrap();
            }
            _ => {
                if (cycles_done + 1) % 40 == 20 {
                    signal_strength += (cycles_done + 1) * reg;
                }
                draw_pixel(&mut crt, reg, cycles_done + 1);
                cycles_done += 1;
            }
        }
    }
    println!("ans1 = {signal_strength}");
    for row in crt {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

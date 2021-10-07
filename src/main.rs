use rand::{self, Rng};

const FREE: u8 = 0;
const WALL: u8 = 1;
const WIDTH: usize = 90;
const HEIGHT: usize = 90;
const FILL: u8 = 45;

struct SettingsMap{
    r1: u8,
    r2: u8,
    reps: u8,
}

impl SettingsMap {
    fn new(r11: u8, r22: u8, repss: u8) -> SettingsMap {
        SettingsMap {
            r1: r11,
            r2: r22,
            reps: repss,
        }
    }
}

fn rand_pick() -> u8{
    if rand::thread_rng()
        .gen_range(0..=100) < FILL {return WALL} FREE
}

fn initmap() -> ([[u8; WIDTH]; HEIGHT], [[u8; WIDTH]; HEIGHT]){
    let mut a = [[1_u8; WIDTH]; HEIGHT];
    let b = [[1_u8; WIDTH]; HEIGHT];

    for row in a.iter_mut() {
        for col in row.iter_mut() {
            *col = rand_pick();
        }
    }

    for y in 0..HEIGHT {
        a[y][0] = WALL;
        a[y][WIDTH - 1] = WALL;
    }

    for x in 0..WIDTH {
        a[0][x] = WALL;
        a[HEIGHT - 1][x] = WALL;
    }
    (a, b)
}

fn generation(a : &mut [[u8; WIDTH]; HEIGHT], b : &mut [[u8; WIDTH]; HEIGHT], set_map: &SettingsMap){
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let mut r1 = 0;
            let mut r2 = 0;

            for i in 0..=2 {
                for j in 0..=2 {
                    if a[y + i - 1][x + j - 1] == WALL {
                        r1 += 1;
                    }
                }
            }

            for i in y..y + 4 {
                for j in x..x + 4 {
                    if ((i - y) == 4 || (i - y) == 0) && ((j - x) == 4 || (j - x) == 0) {
                        continue;
                    }

                    if i >= HEIGHT || j >= WIDTH || i < 2 || j < 2 {
                        continue;
                    }

                    if a[i - 2][j - 2] == WALL {
                        r2 += 1;
                    } 
                }
            }

            if r1 >= set_map.r1 || r2 <= set_map.r2 {
                b[y][x] = WALL;
            } else {
                b[y][x] = FREE;
            }
        }
    }
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            a[y][x] = b[y][x];
        }
    }
}

fn print_map(a : & [[u8; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if a[y][x] == WALL {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let set_map11 = SettingsMap::new(5, 0, 3);
    let (mut a, mut b) = initmap();
    print_map(& a);
    print!("\n\n\n");
    for _ in 0..set_map11.reps {
        generation(&mut a, &mut b, &set_map11);
    }
    print_map(& a);
}

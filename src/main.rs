use rand::{self, Rng}; 

const FREE: u8 = 0;
const WALL: u8 = 1;

struct Map{
    map: Vec<Vec<u8>>,
    temp_array: Vec<Vec<u8>>,
    y_max: usize, 
    x_max: usize,
    fill: u8,
}

impl Map {
    fn rand_pick(fill: u8) -> u8{
        if rand::thread_rng()
            .gen_range(0..=100) < fill {return WALL} FREE
    }

    fn initmap(y_max: usize, x_max: usize, fill: u8) -> Map {
        let mut a = vec![vec![1_u8; x_max]; y_max];
        let b = vec![vec![1_u8; x_max]; y_max];
    
        for row in a.iter_mut() {
            for col in row.iter_mut() {
                *col = Map::rand_pick(fill);
            }
        }
    
        for y in 0..y_max {
            a[y][0] = WALL;
            a[y][x_max - 1] = WALL;
        }
    
        for x in 0..x_max {
            a[0][x] = WALL;
            a[y_max - 1][x] = WALL;
        }

        Map {
            map: a,
            temp_array: b,
            y_max,
            x_max,
            fill,
        }
    }

    fn generation(&mut self, set_map: &SettingsMap){
        for y in 1..self.y_max - 1 {
            for x in 1..self.x_max - 1 {
                let mut r1 = 0;
                let mut r2 = 0;
    
                for i in 0..=2 {
                    for j in 0..=2 {
                        if self.map[y + i - 1][x + j - 1] == WALL {
                            r1 += 1;
                        }
                    }
                }
    
                for i in y..y + 4 {
                    for j in x..x + 4 {
                        if ((i - y) == 4 || (i - y) == 0) && ((j - x) == 4 || (j - x) == 0) {
                            continue;
                        }
    
                        if i >= self.y_max || j >= self.x_max || i < 2 || j < 2 {
                            continue;
                        }
    
                        if self.map[i - 2][j - 2] == WALL {
                            r2 += 1;
                        } 
                    }
                }
    
                if r1 >= set_map.r1 || r2 <= set_map.r2 {
                    self.temp_array[y][x] = WALL;
                } else {
                    self.temp_array[y][x] = FREE;
                }
            }
        }
        for y in 1..self.y_max - 1 {
            for x in 1..self.x_max - 1 {
                self.map[y][x] = self.temp_array[y][x];
            }
        }
    }

    fn print_map(&self) {
        for row in self.map.iter() {
            for col in row.iter() {
                if *col == WALL {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

struct SettingsMap{
    r1: u8,
    r2: u8,
    reps: u8,
}

impl SettingsMap {
    fn new(r1: u8, r2: u8, reps: u8) -> SettingsMap {
        SettingsMap {
            r1,
            r2,
            reps,
        }
    }
}

fn main() {
    let set_ant_map = SettingsMap::new(5, 0, 3);
    let (y_max, x_max, fill) = (90, 90, 45);
    let mut ant_map = Map::initmap(y_max, x_max, fill);

    for _ in 0..set_ant_map.reps {
        ant_map.generation(&set_ant_map);
    }

    ant_map.print_map();
}
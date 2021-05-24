use std::collections::HashSet;

pub const FIELD_WIDTH: usize = 6;
pub const FIELD_HEIGHT: usize = 5;

#[derive(Debug, Default, Clone, Hash)]
pub struct Drop {
    pub drop_type:i32,
    // pub is_delete: std::cell::Cell<bool>,
    pub is_search: bool,
    pub combo_hash: String,
}

impl Drop {
    pub fn drop_type_mut(&mut self, a: i32) {
        self.drop_type = a;
    }

    pub fn is_search_mut(&mut self, a:bool) {
        self.is_search = a;
    }

    pub fn combo_hash_mut(&mut self, a: String) {
        self.combo_hash = a;
    }
}

#[derive(Debug, Hash)]
pub struct Puzzle {
    pub field: [[Drop; FIELD_WIDTH]; FIELD_HEIGHT],
    pub field_width: i32,
    pub field_height: i32,
}

impl Puzzle {
    // pub fn show(&self) {
    //     for i in self.field.iter() {
    //         for j in i.iter() {
    //             // print!("{} ", j.combo_hash.borrow().to_string())
    //             print!("{:02} ", j.drop_type.get())
    //         }
    //         println!()
    //     }
    // }

    fn down_drop(&mut self, x:usize, y:usize) {
        if self.field[y+1][x].combo_hash != "" {
            let tmp1 = self.field[y][x].drop_type;
            self.field[y][x].drop_type_mut(self.field[y+1][x].drop_type);
            self.field[y+1][x].drop_type_mut(tmp1);

            let tmp2 = self.field[y][x].is_search;
            self.field[y][x].is_search_mut(self.field[y+1][x].is_search);
            self.field[y+1][x].is_search_mut(tmp2);

            let tmp3 = self.field[y][x].combo_hash.clone();
            self.field[y][x].combo_hash_mut(self.field[y+1][x].combo_hash.clone());
            self.field[y+1][x].combo_hash_mut(tmp3);

            if y as i32 +1 < self.field_height-1 {
                self.down_drop(x, y+1)
            }
        }
    }

    pub fn next_field(&mut self) {
        for y in (0..self.field_height-2).rev() {
            for x in (0..self.field_width-1).rev() {
                self.down_drop(x as usize, y as usize);
            }
        }
        for y in 0..self.field_height as usize {
            for x in 0..self.field_width as usize {
                if self.field[y][x].combo_hash != "" {
                    self.field[y][x].drop_type_mut(-1);
                }
                self.field[y][x].is_search_mut(false);
                self.field[y][x].combo_hash_mut("".to_string());
            }
        }
    }

    pub fn check_drop(&mut self, x:usize, y:usize, drop_type:i32, hash:String) {
        if drop_type != self.field[y][x].drop_type || self.field[y][x].is_search || self.field[y][x].drop_type == -1 {
            return
        } else {
            self.field[y][x].is_search_mut(true);
            let xx: i32 = x.clone() as i32;
            let yy: i32 = y.clone() as i32;
            if xx -1 >= 0 && xx +1 < self.field_width {
                if self.field[y][x-1].drop_type == drop_type && self.field[y][x+1].drop_type == drop_type {
                    self.field[y][x-1].combo_hash_mut(hash.clone());
                    self.field[y][x].combo_hash_mut(hash.clone());
                    self.field[y][x+1].combo_hash_mut(hash.clone());
                }
            }
            if yy -1 >= 0 && yy +1 < self.field_height {
                if self.field[y-1][x].drop_type == drop_type && self.field[y+1][x].drop_type == drop_type {
                    self.field[y-1][x].combo_hash_mut(hash.clone());
                    self.field[y][x].combo_hash_mut(hash.clone());
                    self.field[y+1][x].combo_hash_mut(hash.clone());
                }
            }
            if xx-1 >= 0 { self.check_drop(x-1, y, drop_type, hash.clone()) } // 左
            if xx+1 < self.field_width { self.check_drop(x+1, y, drop_type, hash.clone()) } // 右
            if yy-1 >= 0 { self.check_drop(x, y-1, drop_type, hash.clone()) } // 上
            if yy+1 < self.field_height { self.check_drop(x, y+1, drop_type, hash.clone()) } // 下
        }
    }

    pub fn get_combo(&mut self) -> i32 {
        let mut combo: i32 = 0;
        loop {
            for y in 0..FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    self.check_drop(x, y, self.field[y][x].drop_type, format!("{}{}", x, y));
                }
            }
    
            let mut combo_hash: HashSet<String> = HashSet::new();
            for row in self.field.iter() {
                for col in row.iter() {
                    if col.combo_hash != "" {
                        combo_hash.insert(col.combo_hash.clone());
                    }
                }
            }
            if combo_hash.len() == 0 {
                break;
            }
            combo += combo_hash.len() as i32;
            self.next_field();
        }
        combo
    }
}
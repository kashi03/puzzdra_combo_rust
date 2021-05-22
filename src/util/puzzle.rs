use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Drop {
    pub drop_type: std::cell::Cell<i32>,
    // pub is_delete: std::cell::Cell<bool>,
    pub is_search: std::cell::Cell<bool>,
    pub combo_hash: std::cell::RefCell<String>,
}

#[derive(Debug)]
pub struct Puzzle {
    pub field: [[Drop; 6]; 5],
    pub field_width: i32,
    pub field_height: i32,
}

impl Puzzle {
    pub fn show(&self) {
        for i in self.field.iter() {
            for j in i.iter() {
                // print!("{} ", j.combo_hash.borrow().to_string())
                print!("{:02} ", j.drop_type.get())
            }
            println!()
        }
    }

    fn down_drop(&self, x:usize, y:usize) {
        if self.field[y+1][x].combo_hash.borrow().to_string() != "" {
            let tmp1 = self.field[y][x].drop_type.get();
            let tmp2 = self.field[y+1][x].drop_type.get();
            self.field[y][x].drop_type.set(tmp2);
            self.field[y+1][x].drop_type.set(tmp1);
    
            let tmp3 = self.field[y][x].is_search.get();
            let tmp4 = self.field[y+1][x].is_search.get();
            self.field[y][x].is_search.set(tmp4);
            self.field[y+1][x].is_search.set(tmp3);
    
            let tmp5 = self.field[y][x].combo_hash.borrow().to_string();
            let tmp6 = self.field[y+1][x].combo_hash.borrow().to_string();
            self.field[y][x].combo_hash.replace(tmp6);
            self.field[y+1][x].combo_hash.replace(tmp5);
            if y as i32 +1 < self.field_height-1 {
                self.down_drop(x, y+1)
            }
        }
    }

    fn next_field(&self) {
        for y in (0..self.field_height-2).rev() {
            for x in (0..self.field_width-1).rev() {
                self.down_drop(x as usize, y as usize);
            }
        }
        for y in 0..self.field_height as usize {
            for x in 0..self.field_width as usize {
                if self.field[y][x].combo_hash.borrow().to_string() != "" {
                    self.field[y][x].drop_type.set(-1);
                }
                self.field[y][x].is_search.set(false);
                self.field[y][x].combo_hash.replace("".to_string());
            }
        }
    }

    fn check_drop(&self, x:usize, y:usize, drop_type:i32, hash:String) {
        if drop_type != self.field[y][x].drop_type.get() || self.field[y][x].is_search.get() || self.field[y][x].drop_type.get() == -1 {
            return
        } else {
            self.field[y][x].is_search.set(true);
            let xx: i32 = x.clone() as i32;
            let yy: i32 = y.clone() as i32;
            if xx -1 >= 0 && xx +1 < self.field_width {
                if self.field[y][x-1].drop_type.get()  == drop_type && self.field[y][x+1].drop_type.get() == drop_type {
                    self.field[y][x-1].combo_hash.replace(hash.clone());
                    self.field[y][x].combo_hash.replace(hash.clone());
                    self.field[y][x+1].combo_hash.replace(hash.clone());
                }
            }
            if yy -1 >= 0 && yy +1 < self.field_height {
                if self.field[y-1][x].drop_type.get()  == drop_type && self.field[y+1][x].drop_type.get() == drop_type {
                    self.field[y-1][x].combo_hash.replace(hash.clone());
                    self.field[y][x].combo_hash.replace(hash.clone());
                    self.field[y+1][x].combo_hash.replace(hash.clone());
                }
            }
            if xx-1 >= 0 { self.check_drop(x-1, y, drop_type, hash.clone()) } // 左
            if xx+1 < self.field_width { self.check_drop(x+1, y, drop_type, hash.clone()) } // 右
            if yy-1 >= 0 { self.check_drop(x, y-1, drop_type, hash.clone()) } // 上
            if yy+1 < self.field_height { self.check_drop(x, y+1, drop_type, hash.clone()) } // 下
        }
    }

    pub fn get_combo(&self) -> i32 {
        let mut combo: i32 = 0;
        loop {
            for (y, row) in self.field.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    self.check_drop(x, y, col.drop_type.get(), format!("{}{}", x, y));
                }
            }

            let mut combo_hash: HashSet<String> = HashSet::new();
            for row in self.field.iter() {
                for col in row.iter() {
                    if col.combo_hash.borrow().to_string() != "" {
                        combo_hash.insert(col.combo_hash.borrow().to_string());
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
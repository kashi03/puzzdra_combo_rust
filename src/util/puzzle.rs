use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Drop {
    pub p_type: i32,
    pub is_delete: bool,
    pub is_search: bool,
    pub combo_hash: String,
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
                print!("{} ", j.p_type)
            }
            println!()
        }
    }

    fn next_field(&self) {

    }

    fn check_drop(&self, x:usize, y:usize, p_type:i32, hash:String) {

    }

    pub fn get_combo(&self) -> i32 {
        let mut combo: i32 = 0;
        loop {
            for (y, row) in self.field.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    self.check_drop(x, y, col.p_type, format!("{}{}", x, y));
                }
            }

            let mut combo_hash: HashSet<String> = HashSet::new();
            for row in self.field.iter() {
                for col in row.iter() {
                    if col.combo_hash != "" {
                        combo_hash.insert((*col.combo_hash).to_string());
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
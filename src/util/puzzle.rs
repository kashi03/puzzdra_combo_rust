#[derive(Debug)]
#[derive(Default)]
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
}
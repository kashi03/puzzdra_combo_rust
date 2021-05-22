mod util;
use util::puzzle as pz;
use std::cell::{Cell, RefCell};

fn main() {
    let f: [[i32;6];5] = [
        [0,0,0,3,4,5],
        [1,2,3,4,5,0],
        [2,3,3,5,0,1],
        [2,2,2,0,1,2],
        [2,5,3,1,2,3],
    ];
    let mut field: [[pz::Drop; 6]; 5] = Default::default();
    for (i, row) in f.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            field[i][j] = pz::Drop {
                drop_type: Cell::new(*col),
                // is_delete: Cell::new(false),
                is_search: Cell::new(false),
                combo_hash: RefCell::new("".to_string()),
            }
        }
    }

    let a = pz::Puzzle {
        field: field,
        field_width: 6,
        field_height: 5,
    };
    let combo = a.get_combo();
    a.show();
    println!("{}", combo)
}

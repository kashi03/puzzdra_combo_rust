mod util;
use util::puzzle as pz;

fn main() {
    let f: [[i32;6];5] = [
        [0,1,2,3,4,5],
        [1,2,3,4,5,0],
        [2,3,4,5,0,1],
        [3,4,5,0,1,2],
        [4,5,0,1,2,3],
    ];
    let mut field: [[pz::Drop; 6]; 5] = Default::default();
    for (i, row) in f.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            field[i][j] = pz::Drop {
                p_type: *col,
                is_delete: false,
                is_search: false,
                combo_hash: String::from(""),
            }
        }
    }

    let a = pz::Puzzle {
        field: field,
        field_width: 6,
        field_height: 5,
    };
    a.show();
}

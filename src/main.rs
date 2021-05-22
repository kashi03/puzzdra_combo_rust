mod util;
use util::puzzle as pz;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Default, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Default, Clone)]
struct State {
    field: [[pz::Drop; 6]; 5],
    combo: i32,
    point: Point,
    move_history: Vec<Point>
}

impl PartialEq for State {
    fn eq(&self, other:&Self) -> bool {
        self.combo == other.combo
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.combo.cmp(&other.combo))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.combo.cmp(&other.combo)
    }
}

fn get_next_state(state: State, dst: Point) -> State {
    let next_field: [[pz::Drop; 6]; 5] = Default::default();
    for (y, row) in next_field.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            col.drop_type.set(state.field[y][x].drop_type.get());
        }
    }

    let tmp1 = next_field[state.point.y as usize][state.point.x as usize].drop_type.get();
    let tmp2 = next_field[dst.y as usize][dst.x as usize].drop_type.get();
    state.field[state.point.y as usize][state.point.x as usize].drop_type.set(tmp2);
    state.field[dst.y as usize][dst.x as usize].drop_type.set(tmp1);
    next_field[state.point.y as usize][state.point.x as usize].drop_type.set(tmp2);
    next_field[dst.y as usize][dst.x as usize].drop_type.set(tmp1);
    let puzzle = pz::Puzzle {
        field: state.field,
        field_width: 6,
        field_height: 5,
    };
    let combo = puzzle.get_combo();
    let mut next_move_histry: Vec<Point> = Vec::new();
    next_move_histry.append(&mut state.move_history.clone());
    next_move_histry.push(dst.clone());
    State {
        field: next_field,
        combo: combo,
        point: dst.clone(),
        move_history: next_move_histry,
    }
}

fn beam_search(first_state: State, move_number: usize) -> State {
    let field_width = 6;
    let field_height = 5;
    let k = 1000; // ビーム幅
    let mut now_states:BinaryHeap<State> = BinaryHeap::new();
    now_states.push(first_state);
    
    for _ in 0..move_number {
        println!("{}", now_states.len());
        let mut next_states:BinaryHeap<State> = BinaryHeap::new();
        for _ in 0..k {
            if now_states.is_empty() { break; }
            let state: State = now_states.pop().unwrap();
            if state.point.x-1 >= 0 {
                let next_state = get_next_state(state.clone(), Point { x: state.point.x-1, y: state.point.y });
                next_states.push(next_state);
            }
            if state.point.x+1 < field_width {
                let next_state = get_next_state(state.clone(), Point { x: state.point.x+1, y: state.point.y });
                next_states.push(next_state);
            }
            if state.point.y-1 >= 0 {
                let next_state = get_next_state(state.clone(), Point { x: state.point.x, y: state.point.y-1 });
                next_states.push(next_state);
            }
            if state.point.y+1 < field_height {
                let next_state = get_next_state(state.clone(), Point { x: state.point.x, y: state.point.y+1 });
                next_states.push(next_state);
            }
        }
        now_states = next_states;
    }
    now_states.pop().unwrap()
}

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

    let first_point: Point = Default::default();
    let first_state: State = State {
        field: field,
        combo: 0,
        point: first_point.clone(),
        move_history: vec![first_point.clone()]
    };
    let combo = beam_search(first_state, 20);
    // let a = pz::Puzzle {
    //     field: field,
    //     field_width: 6,
    //     field_height: 5,
    // };
    // let combo = a.get_combo();
    // a.show();
    println!("{}", combo.combo)
}

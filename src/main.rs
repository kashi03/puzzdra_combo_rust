mod util;
use util::puzzle as pz;
use util::puzzle::{ FIELD_WIDTH, FIELD_HEIGHT };
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use rand::{ thread_rng, Rng };

#[derive(Debug, Default, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Default, Clone)]
struct State {
    field: [[pz::Drop; FIELD_WIDTH]; FIELD_HEIGHT],
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

fn get_next_state(state:&mut State, dst: Point) -> State {
    let mut next_field: [[pz::Drop; FIELD_WIDTH]; FIELD_HEIGHT] = Default::default();
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            next_field[y][x].drop_type_mut(state.field[y][x].drop_type.clone());
        }
    }

    let tmp1 = next_field[state.point.y as usize][state.point.x as usize].drop_type;
    let tmp2 = next_field[dst.y as usize][dst.x as usize].drop_type;
    state.field[state.point.y as usize][state.point.x as usize].drop_type_mut(tmp2);
    state.field[dst.y as usize][dst.x as usize].drop_type_mut(tmp1);
    next_field[state.point.y as usize][state.point.x as usize].drop_type_mut(tmp2);
    next_field[dst.y as usize][dst.x as usize].drop_type_mut(tmp1);
    let mut puzzle = pz::Puzzle {
        field: state.field.clone(),
        field_width: FIELD_WIDTH as i32,
        field_height: FIELD_HEIGHT as i32,
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
    let k = 3000; // ビーム幅
    let mut now_states:BinaryHeap<State> = BinaryHeap::new();
    for x in 0..6 {
        for y in 0..5 {
            now_states.push(State {
                field: first_state.field.clone(),
                combo: 0,
                point: Point { x: x, y: y },
                move_history: vec![Point { x: x, y: y }]
            })
        }
    }
    // now_states.push(first_state);
    
    for _count in 0..move_number {
        println!("{}", _count);
        let mut next_states:BinaryHeap<State> = BinaryHeap::new();
        for _ in 0..k {
            if now_states.is_empty() { break; }
            let state: State = now_states.pop().unwrap();
            if state.point.x-1 >= 0 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x-1, y: state.point.y });
                next_states.push(next_state);
            }
            if state.point.x+1 < FIELD_WIDTH as i32 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x+1, y: state.point.y });
                next_states.push(next_state);
            }
            if state.point.y-1 >= 0 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x, y: state.point.y-1 });
                next_states.push(next_state);
            }
            if state.point.y+1 < FIELD_HEIGHT as i32 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x, y: state.point.y+1 });
                next_states.push(next_state);
            }
        }
        now_states = next_states;
    }
    now_states.pop().unwrap()
}

fn main() {
    let mut f: [[i32; FIELD_WIDTH]; FIELD_HEIGHT] = Default::default();
    let mut rng = thread_rng();
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            f[y][x] = rng.gen_range(0..=5);
        }
    }

    let mut field: [[pz::Drop; FIELD_WIDTH]; FIELD_HEIGHT] = Default::default();
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            field[y][x] = pz::Drop {
                drop_type: f[y][x],
                is_search: false,
                combo_hash: "".to_string(),
            }
        }
    }
    let first_point: Point = Point {
        x: 0,
        y: 0
    };
    let first_state: State = State {
        field: field,
        combo: 0,
        point: first_point.clone(),
        move_history: vec![first_point.clone()]
    };
    let combo = beam_search(first_state, 30);
    // let a = pz::Puzzle {
    //     field: field,
    //     field_width: 6,
    //     field_height: 5,
    // };
    // let combo = a.get_combo();
    // a.show();
    for i in f.iter() {
        println!("{:?}", i);
    }
    println!("{:?}", combo.move_history);
    println!("{}", combo.combo);
}

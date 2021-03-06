mod util;
use util::puzzle as pz;
use util::puzzle::{ FIELD_WIDTH, FIELD_HEIGHT };
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rand::{ thread_rng, Rng };


#[derive(Debug, Default, Clone, Hash)]
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

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field.hash(state);
        self.combo.hash(state);
        self.point.hash(state);
    }
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

fn get_hash(item: &State) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

fn beam_search(first_state: State, move_number: usize) -> State {
    let k = 3000; // ????????????
    let mut now_states:BinaryHeap<State> = BinaryHeap::new();
    for x in 0..6 {
        for y in 0..5 {
            now_states.push(State {
                field: first_state.field.clone(),
                combo: 0,
                point: Point { x: x, y: y },
                move_history: vec![Point { x: x, y: y }]
            });
        }
    }

    let mut max_combo = 0;
    let mut best_state: State = Default::default();
    let mut done: HashSet<u64> = HashSet::new();
    // now_states.push(first_state);
    
    for _count in 0..move_number {
        println!("{}: {}", _count, now_states.len());
        let mut next_states:BinaryHeap<State> = BinaryHeap::new();
        for _ in 0..k {
            if now_states.is_empty() { break; }
            let state: State = now_states.pop().unwrap();
            if max_combo < state.combo {
                max_combo = state.combo;
                best_state = state.clone();
            }
            if state.point.x-1 >= 0 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x-1, y: state.point.y });
                let hash = get_hash(&next_state);
                if !done.contains(&hash) {
                    next_states.push(next_state);
                    done.insert(hash);
                }
            }
            if state.point.x+1 < FIELD_WIDTH as i32 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x+1, y: state.point.y });
                let hash = get_hash(&next_state);
                if !done.contains(&hash) {
                    next_states.push(next_state);
                    done.insert(hash);
                }
            }
            if state.point.y-1 >= 0 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x, y: state.point.y-1 });
                let hash = get_hash(&next_state);
                if !done.contains(&hash) {
                    next_states.push(next_state);
                    done.insert(hash);
                }
            }
            if state.point.y+1 < FIELD_HEIGHT as i32 {
                let next_state = get_next_state(&mut state.clone(), Point { x: state.point.x, y: state.point.y+1 });
                let hash = get_hash(&next_state);
                if !done.contains(&hash) {
                    next_states.push(next_state);
                    done.insert(hash);
                }
            }
        }
        now_states = next_states;
    }
    // now_states.pop().unwrap()
    best_state
}

fn main() {
    // let f: [[i32; FIELD_WIDTH]; FIELD_HEIGHT] = [
    //     [4, 4, 3, 5, 0, 1],
    //     [1, 2, 0, 5, 2, 1],
    //     [1, 5, 5, 3, 4, 0],
    //     [5, 2, 3, 1, 0, 3],
    //     [0, 3, 2, 5, 4, 2],
    // ];
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
    for i in f.iter() {
        println!("{:?}", i);
    }
    println!("{:?}", combo.move_history);
    println!("{}", combo.move_history.len());
    println!("{}", combo.combo);
}

use std::{fs::File, io::Read};
use std::{io, char};

#[derive(Debug)]
#[derive(Clone)]
struct Amanda {
    //current_field_id: i32,
    path: Vec<(i32, i32)>, //id pozicije i broj kljuceva u tom trenutku
    keys: i32,
}

impl Amanda {
    fn new() -> Self {
        Self {
            //current_field_id: 0,
            keys: 0,
            path: Vec::new(),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Field {
    id: i32,
    doors: [bool; 4],
    key: bool,
    exit: bool,
    left: Option<Box<Field>>,
    right: Option<Box<Field>>,
    up: Option<Box<Field>>,
    down: Option<Box<Field>>,
}

impl Field {
    fn new(id: i32, doors: [bool; 4], key: bool, exit:bool) -> Self {
        Self { 
            id: id, 
            doors: doors, 
            key: key, 
            exit: exit, 
            left: None, 
            right: None, 
            up: None, 
            down: None
        }
    }
}

fn main() {
    //iscitati svako polje iz fajla i staviti u vektor
    let result = read_file();
    let binding = result.unwrap();
    let vec: Vec<&str> = binding.split('\n').collect();

    //pravljenje lavirinta kao vektora sa poljima koja pokazuju jedno na drugo
    let mut maze = make_maze(&vec);
    let mut amanda = Amanda::new();
    //trazenje najbolje putanje
    let mut finish_path: Vec<(i32, i32)> = Vec::new();

    //search_for_exit(&mut amanda, 0, &mut maze, &mut finish_path);
    //print!("STARI NACIN: {:?}", finish_path);

    //vektor koje su sve putanje prosle, i broj kljuceva za svaku (buduca pozicija, kljucevi, [istorija putanje = (pozicija, broj kljuceva)])
    let mut path_queue:Vec<(i32, i32, Vec<(i32, i32)>)> = Vec::new();

    //krajnja putanja treba da sadrzi pozicije i broj kljuceva
    let mut finish_path2: Vec<(i32, i32)> = Vec::new();
    search_for_exit2(&mut path_queue, &mut finish_path2, &mut maze);
    println!("Finish path: {:?}", finish_path2);
    print_like_matrix(&finish_path2, &maze);
}

fn search_for_exit2(path_queue:&mut Vec<(i32, i32, Vec<(i32, i32)>)>, finish_path:&mut Vec<(i32, i32)>, maze: &mut Vec<Field>) {
    let mut current_path: Vec<(i32, i32)> = Vec::new();
    let mut current_field = get_from_maze_by_id(maze, 0).unwrap();
    let mut current_keys = 0;

    if path_queue.len() != 0 {
        let path_pom = path_queue.remove(0);
        current_field = get_from_maze_by_id(maze, path_pom.0).unwrap();
        current_keys = path_pom.1;
        current_path = path_pom.2;
    }

    if current_field.exit {
        if finish_path.len() == 0 || finish_path.len() > current_path.len() {
            *finish_path = current_path.clone();
            finish_path.push((current_field.id, current_keys));
        }
    }

    match current_field.up {
        Some(up_field)=> {
            if !current_field.doors[2] {
                let mut update_current_path = current_path.clone();
                update_current_path.push((current_field.id, current_keys));
                let mut update_keys = current_keys;
                if up_field.key && !current_path.iter().any(|&el| el.0 == up_field.id) {
                    update_keys += 1;
                }
                let new_queue_element = (up_field.id, update_keys, update_current_path); 
                if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                    path_queue.push(new_queue_element);
                }
            } else {
                if current_keys > 0 {
                    let mut update_current_path = current_path.clone();
                    update_current_path.push((current_field.id, current_keys));
                    let mut update_keys = current_keys;
                    if up_field.key && !current_path.iter().any(|&el| el.0 == up_field.id) {
                        update_keys += 1;
                    }
                    let new_queue_element = (up_field.id, update_keys-1, update_current_path); 
                    if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                        path_queue.push(new_queue_element);
                    }
                }
            }
        },
        None => {}
    }

    match current_field.down {
        Some(down_field)=> {
            if !current_field.doors[3] {
                let mut update_current_path = current_path.clone();
                update_current_path.push((current_field.id, current_keys));
                let mut update_keys = current_keys;
                if down_field.key && !current_path.iter().any(|&el| el.0 == down_field.id) {
                    update_keys += 1;
                }
                let new_queue_element = (down_field.id, update_keys, update_current_path); 
                if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                    path_queue.push(new_queue_element);
                }
            } else {
                if current_keys > 0 {
                    let mut update_current_path = current_path.clone();
                    update_current_path.push((current_field.id, current_keys));
                    let mut update_keys = current_keys;
                    if down_field.key && !current_path.iter().any(|&el| el.0 == down_field.id) {
                        update_keys += 1;
                    }
                    let new_queue_element = (down_field.id, update_keys-1, update_current_path); 
                    if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                        path_queue.push(new_queue_element);
                    }
                }
            }
        },
        None => {}
    }

    match current_field.left {
        Some(left_field)=> {
            if !current_field.doors[0] {
                let mut update_current_path = current_path.clone();
                update_current_path.push((current_field.id, current_keys));
                let mut update_keys = current_keys;
                if left_field.key && !current_path.iter().any(|&el| el.0 == left_field.id) {
                    update_keys += 1;
                }
                let new_queue_element = (left_field.id, update_keys, update_current_path); 
                if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                    path_queue.push(new_queue_element);
                }
            } else {
                if current_keys > 0 {
                    let mut update_current_path = current_path.clone();
                    update_current_path.push((current_field.id, current_keys));
                    let mut update_keys = current_keys;
                    if left_field.key && !current_path.iter().any(|&el| el.0 == left_field.id) {
                        update_keys += 1;
                    }
                    let new_queue_element = (left_field.id, update_keys-1, update_current_path); 
                    if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                        path_queue.push(new_queue_element);
                    }
                }
            }
        },
        None => {}
    }
    
    
    match current_field.right {
        Some(right_field)=> {
            if !current_field.doors[1] {
                let mut update_current_path = current_path.clone();
                update_current_path.push((current_field.id, current_keys));
                let new_queue_element = (right_field.id, current_keys, update_current_path); 
                if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                    path_queue.push(new_queue_element);
                }
            } else {
                if current_keys > 0 {
                    let mut update_current_path = current_path.clone();
                    update_current_path.push((current_field.id, current_keys));
                    let new_queue_element = (right_field.id, current_keys-1, update_current_path); 
                    if !current_path.iter().any(|&el| el == (new_queue_element.0, new_queue_element.1)) { //provera da li smo vec bili tu sa istim brojem kljuceva
                        path_queue.push(new_queue_element);
                    }
                }
            }
        },
        None => {}
    }

    if path_queue.is_empty() {
        return;
    }

    search_for_exit2(path_queue, finish_path, maze);


}

fn print_like_matrix(path: &Vec<(i32, i32)>, maze: &Vec<Field>) {
    let mut solution_maze:Vec<i32> = Vec::new();
    for field in maze {
        if path.iter().any(|&el| el.0 == field.id) {
            solution_maze.push(1);
        } else {
            solution_maze.push(0);
        }
    }

    for (idx, field) in solution_maze.iter().enumerate() {
        if idx % 9 != 0 {
            print!(" {} ", field);
        } else {
            print!("\n\n {} ", field);
        }
    }
}

fn search_for_exit(amanda: &mut Amanda, field_id: i32, maze: &mut Vec<Field>, finish_path: &mut Vec<(i32, i32)>) {
    //dobavljamo trenutno polje putem id-a
    let current_field = get_from_maze_by_id(maze, field_id).unwrap();

    //proveriti da li vec postoji zavrsna putanja
    if finish_path.len() > 0 {
        return;
    }

    //proveriti da li ima kljuceva u polju
    if current_field.key == true && !amanda.path.iter().any(|&el| el.0 == current_field.id){
        amanda.keys += 1;
    }
    //dodavanje polja u putanju
    if !amanda.path.iter().any(|&el| el == (current_field.id, amanda.keys)) {
        amanda.path.push((current_field.id, amanda.keys));
    } else {
        return;
    }

    //da li je kraj?
    if current_field.exit {
        *finish_path = amanda.path.clone();
        return;
    }

    //PROVERITI KOJI KORAK DOVODI BLIZE CILJU PA POREDJATI PO REDOSLEDU


    match current_field.down {
        Some(field) => {
            if !current_field.doors[3] {
                search_for_exit(amanda, field.id, maze, finish_path);
            } else {
                if amanda.keys > 0 {
                    amanda.keys -= 1;
                    unlock_doors(maze, 3, current_field.id);
                    unlock_doors(maze, 2, field.id);
                    search_for_exit(amanda, field.id, maze, finish_path);
                }
            }
        },
        None => {}
    }

    match current_field.up {
        Some(field) => {
            if !current_field.doors[2] {
                search_for_exit(amanda, field.id, maze, finish_path);
            } else {
                if amanda.keys > 0 {
                    amanda.keys -= 1;
                    unlock_doors(maze, 2, current_field.id);
                    unlock_doors(maze, 3, field.id);
                    search_for_exit(amanda, field.id, maze, finish_path);
                }
            }
        },
        None => {}
    }

    match current_field.right {
        Some(field) => {
            if !current_field.doors[1] {
                search_for_exit(amanda, field.id, maze, finish_path);
            } else {
                
                if amanda.keys > 0 {
                    amanda.keys -= 1;
                    unlock_doors(maze, 1, current_field.id);
                    unlock_doors(maze, 0, field.id);
                    search_for_exit(amanda, field.id, maze, finish_path);
                }
            }
        },
        None => {}
    }

    //sad pozvati za svakog suseda koji nije None
    match current_field.left {
        Some(field) => {
            if !current_field.doors[0] {
                search_for_exit(amanda, field.id, maze, finish_path);
            } else {
                if amanda.keys > 0 {
                    amanda.keys -= 1;
                    //promeniti u maze-u da bude false nakon otkljucavanja na obe strane
                    unlock_doors(maze, 0, current_field.id);
                    unlock_doors(maze, 1, field.id);
                    search_for_exit(amanda, field.id, maze, finish_path);
                }
            }
        },
        None => {}
    }

    return;
}

fn unlock_doors(maze: &mut Vec<Field>, position: usize, id: i32) {
    for field in maze {
        if field.id == id {
            field.doors[position] = false;
            return;
        }
    }
    return;
}

fn get_from_maze_by_id(maze: &Vec<Field>, id: i32) -> Option<Field> {
    for field in maze.iter() {
        if field.id == id {
            return Some(field.clone());
        }
    }
    return None;
}

fn read_file() -> Result<String, io::Error> {
    let mut content = String::new();

    File::open("maze_def.txt")?.read_to_string(&mut content)?;

    Ok(content)
}

fn make_maze(vector_of_positions: &Vec<&str>) -> Vec<Field> {
    let mut maze: Vec<Field> = Vec::new();
    //id
    let mut index = 0;
    for position in vector_of_positions {
        let position_slice: Vec<&str> = position.split_whitespace().collect();
        //vrata
        let mut door_index = 0;
        let mut door_arr:[bool; 4] = [false, false, false, false];
        for char in position_slice.get(1).unwrap().chars() {
            if char == '0' {
                door_arr[door_index] = false;
            } else {
                door_arr[door_index] = true;
            }
            door_index += 1;
        }
        
        //kljuc i izlaz
        let mut key = false;
        let mut exit = false;
        let mut key_and_exit_index = 0;
        let mut key_exit_arr: [char; 4] = [' ',' ',' ',' '];
        for char1 in position_slice.get(2).unwrap().chars() {
            key_exit_arr[key_and_exit_index] = char1;
            key_and_exit_index += 1;
        }

        if key_exit_arr[0] == '1' && key_exit_arr[1] == '1' {
            key = true;
        }

        if key_exit_arr[2] == '1' && key_exit_arr[3] == '1' {
            exit = true;
        }
        maze.push(Field::new(index, door_arr, key, exit));
        index += 1;
    }
    
    let mut maze_clone = maze.clone();
    for mut field in &mut maze {
        let position = vector_of_positions.get(field.clone().id as usize).unwrap();
        let position_slice: Vec<&str> = position.split_whitespace().collect();
        let mut idx = 0;
        for char in position_slice.get(0).unwrap().chars(){
            if char == '1' {
                if idx == 0 {
                    let id = field.id;
                    field.left = find_field_by_id(&mut maze_clone, id - 1 );
                }
                if idx == 1 {
                    let id = field.id;
                    field.right = find_field_by_id(&mut maze_clone, id + 1 );
                }
                if idx == 2 {
                    let id = field.id;
                    field.up = find_field_by_id(&mut maze_clone, id - 9 );
                }
                if idx == 3 {
                    let id = field.id;
                    field.down = find_field_by_id(&mut maze_clone, id + 9 );
                }
            }
            idx += 1;
        }
    }

    return maze;
}

fn find_field_by_id(maze_vector: &mut Vec<Field>, id: i32) -> Option<Box<Field>>{
    for field in maze_vector {
        if field.id == id {
            return Some(Box::new(field.clone()));
        }
    }
    return None;
}

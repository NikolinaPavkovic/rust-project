use std::{fs::File, io::Read};
use std::{io, char};

#[derive(Debug)]
#[derive(Clone)]
struct Amanda {
    current_field_id: i32,
    path: Vec<(i32, i32)>, //id pozicije i broj kljuceva u tom trenutku
    keys: i32,
}

impl Amanda {
    fn new() -> Self {
        Self {
            current_field_id: 0,
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
    //read every field from file and put to vector
    let result = read_file();
    let binding = result.unwrap();
    let vec: Vec<&str> = binding.split('\n').collect();

    //pravljenje lavirinta kao vektora sa poljima koja pokazuju jedno na drugo
    let mut maze = make_maze(&vec);
    let mut amanda = Amanda::new();
    //trazenje najbolje putanje
    let mut finish_path: Vec<(i32, i32)> = Vec::new();
    search_for_exit(&mut amanda, 0, &mut maze, &mut finish_path);
    print!("{:?}", finish_path);
}

fn search_for_exit(amanda: &mut Amanda, field_id: i32, maze: &mut Vec<Field>, finish_path: &mut Vec<(i32, i32)>) {
    //dobavljamo trenutno polje putem id-a
    let mut current_field = get_from_maze_by_id(maze, field_id).unwrap();

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

    //sad pozvati za svako suseda koji nije None
    match current_field.left {
        Some(field) => {
            if !current_field.doors[0] {
                search_for_exit(amanda, field.id, maze, finish_path);
            } else {
                if amanda.keys > 0 {
                    amanda.keys -= 1;
                    //current_field.doors[0] = false; //promeniti u maze-u da bude false nakon otkljucavanja na obe strane
                    unlock_doors(maze, 0, current_field.id);
                    unlock_doors(maze, 1, field.id);
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

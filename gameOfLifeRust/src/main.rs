/*
Rules:
Any live cell with two or three live neighbours survives.
Any dead cell with three live neighbours becomes a live cell.
All other live cells die in the next generation. Similarly, all other dead cells stay dead.

Current issues:
- Graphics/visualization

*/

use std::time::Duration;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

//Takes vec of live cells and coordinates of one cell
fn get_number_live_neighbors(live_cells: &Vec<[i32; 2]>, row: i32, col: i32) -> i32 {
    let neighbors = [[row-1,col-1], [row,col-1], [row+1,col-1],[row-1,col], [row+1,col],[row-1,col+1], [row,col+1], [row+1,col+1]];
    let mut count = 0;
    for live_cell in live_cells {
        for neighbor in neighbors {
            if *live_cell == neighbor{
                count+=1;
            }
        }
    }

    count
}

fn should_live(number_live_neighbors: i32, is_alive: bool) -> bool {
    if number_live_neighbors == 2 && is_alive {
        return true;
    }
    else if number_live_neighbors == 3 {
        return true;
    }

    false
}

fn is_alive(live_cells: &Vec<[i32; 2]>, row: i32, column: i32) -> bool {
    let coords: [i32; 2] = [row, column];
    for live_cell in live_cells {
        if *live_cell == coords {
            return true;
        }
    }

    false
}

fn print_board(live_cells: &Vec<[i32; 2]>, board_size: i32) {
    for row in 0..board_size {
        for column in 0..board_size {
            if is_alive(live_cells, row, column) {
                print!("X");
            }
            else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

fn update(live_cells: &mut Vec<[i32; 2]>, board_size: i32) -> Vec<[i32; 2]> {
    let mut new_live_cells: Vec<[i32; 2]> = Vec::new();
    for row in 0..board_size {
        for column in 0..board_size {
            let number_of_live_neighbors = get_number_live_neighbors(live_cells, row, column);
            let is_alive = is_alive(live_cells, row, column);
            let should_live = should_live(number_of_live_neighbors, is_alive);
            if should_live {
                let current_coords: [i32; 2] = [row,column];
                new_live_cells.push(current_coords);
            }
        }
    }

    live_cells.clear();
    for cell in new_live_cells {
        live_cells.push(cell);
    }

    print_board(live_cells, board_size);

    std::thread::sleep(Duration::new(1,0));

    live_cells.to_vec()
}

fn main() {
    let board_size = 40;

    let mut live_cells: Vec<[i32; 2]> = Vec::new();

    //glider
    live_cells.push([1,1]);
    live_cells.push([3,1]);
    live_cells.push([2,2]);
    live_cells.push([2,3]);
    live_cells.push([3,2]);

    print_board(&live_cells, board_size);

    loop {
    // let mut new_live_cells: Vec<[i32; 2]> = Vec::new();
    // for row in 0..board_size {
    //     for column in 0..board_size {
    //         let number_of_live_neighbors = get_number_live_neighbors(&live_cells, row, column);
    //         let is_alive = is_alive(&live_cells, row, column);
    //         let should_live = should_live(number_of_live_neighbors, is_alive);
    //         if should_live {
    //             let current_coords: [i32; 2] = [row,column];
    //             new_live_cells.push(current_coords);
    //         }
    //     }
    // }
    //
    // live_cells.clear();
    // for cell in new_live_cells {
    //     live_cells.push(cell);
    // }
    //
    // print_board(&live_cells, board_size);
    //
    // std::thread::sleep(Duration::new(1,0));
    let live_cells = update(&mut live_cells, board_size);

    }
}

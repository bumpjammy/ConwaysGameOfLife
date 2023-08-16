
pub(crate) fn update_board(board: &mut Vec<bool>, generation: &mut usize, width: usize, height: usize) {
    let mut new_board = vec![false; width * height];
    for (index, cell) in board.iter().enumerate() {
        let neighbors = get_neighbours(index as i32, height as i32, width as i32);
        let mut alive_neighbors = 0;
        for neighbor in neighbors {
            if board[neighbor as usize] {
                alive_neighbors += 1;
            }
        }
        if *cell {
            if alive_neighbors == 2 || alive_neighbors == 3 {
                new_board[index] = true;
            }
        } else {
            if alive_neighbors == 3 {
                new_board[index] = true;
            }
        }
    }
    *generation += 1;
    *board = new_board;
}

fn get_neighbours(index: i32, height: i32, width: i32) -> Vec<i32> {
    let mut neighbors = Vec::new();

    let x = index / height;
    let y = index % height;

    let relative_neighbors = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1)
    ];

    for neighbor in relative_neighbors {
        let mut neighbor_x = x + neighbor.0;
        let mut neighbor_y = y + neighbor.1;
        // Deal with wrapping
        if neighbor_x < 0 {
            neighbor_x = width - 1;
        } else if neighbor_x >= width {
            neighbor_x = 0;
        }
        if neighbor_y < 0 {
            neighbor_y = height - 1;
        } else if neighbor_y >= height {
            neighbor_y = 0;
        }

        neighbors.push(neighbor_y + neighbor_x * height);
    }

    neighbors
}
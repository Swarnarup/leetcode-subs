#[derive(Copy, Clone, Debug)]
pub enum Dirs {
    Right,
    Left,
    Up,
    Down
}
use Dirs::*;
impl Solution {
    pub fn has_valid_path(mut grid: Vec<Vec<i32>>) -> bool {
        match grid[0][0] {
            1 => dfs(0, 0, Right, &mut grid),
            2 => dfs(0, 0, Down, &mut grid),
            3 => dfs(0, 0, Right, &mut grid),
            4 => {
                grid[0][0] = -1;
                dfs(0, 1, Right, &mut grid) | dfs(1, 0, Down, &mut grid)
            },
            5 => false,
            6 => dfs(0, 0, Down, &mut grid),
            _ => false
        }
    }
}

fn dfs(i: i32, j: i32, cur_dir: Dirs, grid: &mut Vec<Vec<i32>>) -> bool {
    // println!("{} {} {:?} {}", i, j, cur_dir, grid[i as usize][j as usize]);
    if i + 1 == grid.len() as i32 && j + 1 == grid[0].len() as i32 {
        if next_dir(cur_dir, grid[i as usize][j as usize]).is_some() {
            return true;
        } else {
            return false;
        }
    }
    let tmp = grid[i as usize][j as usize];
    if let Some(nxt_d) = next_dir(cur_dir, grid[i as usize][j as usize]) {
        // println!("{} {} {:?}", i, j, cur_dir);
        grid[i as usize][j as usize] = -1;
        let (di, dj) = next(nxt_d);
        let (ii, jj) = (i+di, j+dj);
        if ii >= 0 && ii < grid.len() as i32 && jj >= 0 && jj < grid[0].len() as i32 {
            if dfs(ii, jj, nxt_d, grid) {
                return true;
            }
        }
    }
    grid[i as usize][j as usize] = tmp;
    false
}




fn next(d: Dirs) -> (i32, i32) {
    match d {
        Right => (0, 1),
        Left => (0, -1),
        Up => (-1, 0),
        Down => (1, 0),
    }
}

fn next_dir(dir: Dirs, state: i32) -> Option<Dirs> {
    match state {
        1 => match dir {
            Right => Some(Right),
            Left => Some(Left),
            Up => None,
            Down => None,
        },
        2 => match dir {
            Right => None,
            Left => None,
            Up => Some(Up),
            Down => Some(Down),
        },
        3 => match dir {
            Right => Some(Down),
            Left => None,
            Up => Some(Left),
            Down => None,
        },
        4 => match dir {
            Right => None,
            Left => Some(Down),
            Up => Some(Right),
            Down => None,
        },
        5 => match dir {
            Right => Some(Up),
            Left => None,
            Up => None,
            Down => Some(Left),
        },
        6 => match dir {
            Right => None,
            Left => Some(Up),
            Up => None,
            Down => Some(Right),
        },
        _ => None,
    }
}
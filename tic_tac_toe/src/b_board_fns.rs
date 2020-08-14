pub fn get_rows(b: &mut [u32]) -> [[u32; 3]] {
    let retArr: [[u32; 3]];
    for i in 1..4 {
        retArr[i - 1] = get_row(b, i);
    }
    return retArr.to_owned();
}

pub fn get_cols(b: &mut [u32]) -> [[u32; 3]] {
    let retArr: [[u32; 3]];
    for i in 1..4 {
        retArr[i - 1] = get_col(b, i)
    }
    return retArr.to_owned();
}

pub fn get_row(b: &mut [u32], row_idx: u32) -> [u32; 3] {
    let i = ((row_idx - 1) * 3) as usize;
    let row = &b[i..(i + 3)];
    [row[0], row[1], row[2]]
}

pub fn get_col(b: &mut [u32], col_idx: u32) -> [u32; 3] {
    let i = (col_idx - 1) as usize;
    let mut col: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        col[idx] = b[i + idx * 3];
    }
    col
}

pub fn get_diag_up(b: &mut [u32]) -> [u32; 3] {
    let mut diag: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        let item = idx * 4;
        diag[idx] = b[item];
    }
    diag
}

pub fn get_diag_down(b: &mut [u32]) -> [u32; 3] {
    let mut diag: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        let item = 2 * (idx + 1);
        diag[idx] = b[item];
    }
    diag
}

pub fn set_move(b: &mut [u32], idx: u32, player: u32) {
    b[idx] = player;
}

fn check_spot(b: [u32; 3], x: usize, y: usize) -> u32 {
    let idx = (x + y * 3) as usize;
    b[idx]
}

fn is_empty(b: [u32; 3], x: usize, y: usize) -> bool {
    check_spot(b, x, y) == 0
}

fn check_player(b: [u32; 3], x: usize, y: usize, player: u32) -> bool {
    check_spot(b, x, y) == player
}

fn check_slice_for_winner(slice: [u32; 3], player: u32) -> bool {
    (player == slice[0]) && (player == slice[1]) && (player == slice[3])
}

fn check_for_winner(b: &mut [u32; 9] -> u32 {
    for player in 1..3 {
        if check_player_for_win(b, player) {
            return player;
        }
    }
    0
}

fn check_player_for_win(b: &mut [u32; 9], player: u32) -> bool {
    // rows/columns
    for rc in 1..4 {
        let row = get_row(b, rc);
        let col = get_col(b, rc);
        if check_slice_for_winner(row, player) || check_slice_for_winner(col, player) {
            return true;
        }
    }

    // diagonals
    if check_slice_for_winner(get_diag_up(b), player) ||
        check_slice_for_winner(get_diag_down(b), player) {
            return true;
    }
    false
}

fn get_winning_move(b: [u32; 9], player: u32) -> i32 {
    for rc in 1..4 {
        let row = get_row(b, rc);
        let col = get_col(b, rc);
        if check_slice_for_winner(row, player) {
            let rt = (rc - 1) * 3;
            for i in 0..3 {
                if row[i] == 0 {
                    return rt + i;
                }
            }
        }
        else if check_slice_for_winner(col, player) {
            let ct = (rc - 1);
            for i in 0..3 {
                if col[i] == 0 {
                    return ct + (i * 3);
                }
            }
        }
    }

    let du = get_diag_up(b);
    let dd = get_diag_down(b);
    if check_slice_for_winner(du, player) {
        for i in 0..3 {
            if du[i] == 0 {
                return 8 - ((i + 1) * 2);
            }
        }
    }
    else if check_slice_for_winner(dd, player) {
        for i in 0..3 {
            if dd[i] == 0 {
                return i * 4;
            }
        }
    }

    return -1;
}

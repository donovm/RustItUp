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

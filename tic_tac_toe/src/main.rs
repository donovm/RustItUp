//let board: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];



fn get_row(b: &mut [u32], row_idx: u32) -> [u32; 3] {
    let i = ((row_idx - 1) * 3) as usize;
    let row = &b[i..(i + 3)];
    [row[0], row[1], row[2]]
}

fn get_col(b: &mut [u32], col_idx: u32) -> [u32; 3] {
    let i = (col_idx - 1) as usize;
    let mut col: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        col[idx] = b[i + idx * 3];
    }
    col
}

fn get_diag_up(b: &mut [u32]) -> [u32; 3] {
    let mut diag: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        let item = idx * 4;
        diag[idx] = b[item];
    }
    diag
}

fn get_diag_down(b: &mut [u32]) -> [u32; 3] {
    let mut diag: [u32; 3] = [0, 0, 0];
    for idx in 0..3 {
        let item = 2 * (idx + 1);
        diag[idx] = b[item];
    }
    diag
}

fn main() {
    let mut a: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for i in 1..4 {
        println!("{:?}", get_row(&mut a, i));
    }

    println!(" ");

    for i in 1..4 {
        println!("{:?}", get_col(&mut a, i));
    }

    println!();
    println!("{:?}", get_diag_up(&mut a));
    println!();
    println!("{:?}", get_diag_down(&mut a));
}

use std::io;
use std::vec::Vec;
use rand::Rng;

pub fn enter_size() -> (usize, usize){
    let res:(usize, usize);
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
    
        let args = s
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        if args[0] <= 0 || args[1] <= 0 {
            continue;
        }
        res = (args[0], args[1]);
        break;
    }
    res
}

pub fn enter_matrix(matrix:&mut Vec<Vec<i32>>){
    for i in 0..matrix.len() {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
    
        let args = s
            .split_whitespace()
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();
        for j in 0..matrix[i].len() {
            matrix[i][j] = args[j];
        }
    }
    println!("\n");
}

pub fn enter_matrix_auto(matrix: &mut Vec<Vec<i32>>){
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            matrix[i][j] = rand::thread_rng().gen_range(0..10);
        }
    }
}

pub fn print_matrix(matrix: &Vec<Vec<i32>>){
    for row in matrix {
        for el in row {
            print!("{} ", el)
        }
        println!()
    }
    println!()
}

pub fn part_matrix(mtrx1: &Vec<Vec<i32>>, mtrx2: &mut Vec<Vec<i32>>){
    let rows = if mtrx1.len() > mtrx2.len() {
        mtrx2.len()
    } else {
        mtrx1.len()
    };

    let cols = if mtrx1[0].len() > mtrx2[0].len() {
        mtrx2[0].len()
    } else {
        mtrx1[0].len()
    };
    
    for i in 0..rows {
        for j in 0..cols {
            mtrx2[i][j] = mtrx1[i][j];
        }
    };
}

pub fn ang_matrix(matrix: &Vec<Vec<i32>>, ang_matrix: &mut Vec<Vec<i32>>, part: i16){
    let mut add_i:usize = 0;
    let mut add_j:usize = 0;
    match part{
        1=> {},
        2=> {
            add_j = matrix.len()/2;
        },
        3=> {
            add_i = matrix.len()/2;
        },
        4=> {
            add_i = matrix.len()/2;
            add_j = matrix.len()/2;
        },
        _=>println!("Wrong part!")
    }
    for i in 0..matrix.len()/2 {
        for j in 0..matrix.len()/2 {
            ang_matrix[i][j] = matrix[i + add_i][j + add_j];
        }
    }
}
pub fn matrix_angs(matrix: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>>{
    let mut matrix_angs: Vec<Vec<Vec<i32>>> = Vec::new();
    for i in 1..=4{
        let mut ang_mtrx = vec![vec![0i32; matrix.len()/2]; matrix.len()/2];
        ang_matrix(&matrix, &mut ang_mtrx, i);
        matrix_angs.push(ang_mtrx);
    }
    matrix_angs
}
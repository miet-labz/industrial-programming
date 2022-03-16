use std::io;
use std::vec::Vec;
use rand::Rng;

fn enter_size() -> (usize, usize){
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
        (args[0], args[1])
    }
}

fn main() {
    println!("{}", usize::min_value());
    println!("Вас приветствует программа");
    println!("Быстрого перемножения матриц методом Штрассена\n");
    ///////////////////////////////////////////////////////////////////////////////
	////////////////////Ввод размеров матрицы пользователем////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    println!("Введите размеры первой матрицы\t");
    let m1_rows:usize;
    let m1_colls:usize;
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
        m1_rows = args[0];
        m1_colls = args[1];
        break;
    }
    println!("Введите размеры второй матрицы\t");
    let m2_rows:usize;
    let m2_colls:usize;
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
        m2_rows = args[0];
        m2_colls = args[1];
        break;
    }
    let mut matrix1 = vec![vec![0i32; m1_colls]; m1_rows];
    let mut matrix2 = vec![vec![0i32; m2_colls]; m2_rows];
    ///////////////////////////////////////////////////////////////////////////////
    ////////////////Выбор способа заполнения и заполнение матриц///////////////////
    ///////////////////////////////////////////////////////////////////////////////
    let option:i32;
    loop {
        println!("Выберите способ заполнения матриц");
        println!("1 - Вручную \n2 - Случайным образом");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim() {
          "1" => option = 1,
          "2" => option = 2,
          _ => continue,
        }
        break
    }
    match option {
        1 => {
            for i in 0..matrix1.len() {
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("Failed to read line");
            
                let args = s
                    .split_whitespace()
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()
                    .unwrap();
                for j in 0..matrix1[i].len() {
                    matrix1[i][j] = args[j];
                }
            }
            println!("\n");
            for i in 0..matrix2.len() {
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("Failed to read line");
        
                let args = s
                    .split_whitespace()
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()
                    .unwrap();
                for j in 0..matrix2[i].len() {
                    matrix2[i][j] = args[j];
                }
            }
            println!("\nМатрица 1\n");
            for row in &matrix1 {
                for el in row {
                    print!("{} ", el)
                }
                print!("\n");
            }
            println!();
            println!("\nМатрица 2\n");
            for row in &matrix2 {
                for el in row {
                    print!("{} ", el)
                }
                print!("\n");
            }
            println!();
        }
        2 => {
            for i in 0..matrix1.len() {
                for j in 0..matrix1[i].len() {
                     matrix1[i][j] = rand::thread_rng().gen_range(0..10);
                }
            }
            for i in 0..matrix2.len() {
                for j in 0..matrix2[i].len() {
                   matrix2[i][j] = rand::thread_rng().gen_range(0..10);
                }
            }
            println!("\nМатрица 1\n");
            for row in &matrix1 {
                for el in row {
                print!("{} ", el)
                }
                print!("\n");
            }
            println!();
            println!("\nМатрица 2\n");
            for row in &matrix2 {
                for el in row {
                print!("{} ", el)
            }
                print!("\n");
            }
            println!();
        }
        _=>println!("Wrong option")
    }
    ///////////////////////////////////////////////////////////////////////////////
	/////////////////Приведение матриц к требуемому размеру////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut aug_size = 2;
    while aug_size < m1_rows || aug_size < m2_rows || aug_size < m1_colls || aug_size < m2_colls {
        aug_size *= 2;
      }
    let mut aug_matrix1 = vec![vec![0i32; aug_size]; aug_size];
    let mut aug_matrix2 = vec![vec![0i32; aug_size]; aug_size];

    let rows = if matrix1.len() > aug_matrix1.len() {
        aug_matrix1.len()
      } else {
        matrix1.len()
      };
      let cols = if matrix1[0].len() > aug_matrix1[0].len() {
        aug_matrix1[0].len()
      } else {
        matrix1[0].len()
      };
    
      for i in 0..rows {
        for j in 0..cols {
            aug_matrix1[i][j] = matrix1[i][j];
        }
    };
    let rows = if matrix2.len() > aug_matrix2.len() {
        aug_matrix2.len()
      } else {
        matrix2.len()
      };
      let cols = if matrix2[0].len() > aug_matrix2[0].len() {
        aug_matrix2[0].len()
      } else {
        matrix2[0].len()
      };
    
      for i in 0..rows {
        for j in 0..cols {
            aug_matrix2[i][j] = matrix2[i][j];
        }
    };


    println!("Приведенные матрицы");

    println!("\nМатрица 1\n");
    for row in &aug_matrix1 {
        for el in row {
          print!("{} ", el)
        }
        print!("\n");
    }
    println!();

    println!("\nМатрица 2\n");
    for row in &aug_matrix2 {
        for el in row {
          print!("{} ", el)
        }
        print!("\n");
    }
    println!();

    ///////////////////////////////////////////////////////////////////////////////
	///////////////Разбиение матриц на подматрицы и их заполнение//////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut ang_matrix1 = vec![vec![0i32; aug_size]; aug_size];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_matrix1[i][j] = aug_matrix1[i][j];
        }
    }
    let mut ang_matrix2 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_matrix2[i][j] = aug_matrix1[i][j + aug_size/2];
        }
    }
    let mut ang_mtrx3 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx3[i][j] = aug_matrix1[i + aug_size/2][j];
        }
    }
    let mut ang_mtrx4 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx4[i][j] = aug_matrix1[i + aug_size/2][j + aug_size/2];
        }
    }
    let mut ang_mtrx5 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx5[i][j] = aug_matrix2[i][j];
        }
    }
    let mut ang_mtrx6 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx6[i][j] = aug_matrix2[i][j + aug_size/2];
        }
    }
    let mut ang_mtrx7 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx7[i][j] = aug_matrix2[i + aug_size/2][j];
        }
    }
    let mut ang_mtrx8 = vec![vec![0i32; aug_size/2]; aug_size/2];
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            ang_mtrx8[i][j] = aug_matrix2[i + aug_size/2][j + aug_size/2];
        }
    }
    
	///////////////////////////////////////////////////////////////////////////////
	////////////////////////Создание промежуточных матриц//////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut interm1 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm2 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm3 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm4 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm5 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm6 = vec![vec![0i32; aug_size]; aug_size];
    let mut interm7 = vec![vec![0i32; aug_size]; aug_size];

    ///////////////////////////////////////////////////////////////////////////////
	////////////////////Вычисление значений промежуточных матриц///////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            interm1[i][j] = 0;
            for z in 0..aug_size/2 {
                interm1[i][j] += (ang_matrix1[i][z] + ang_mtrx4[i][z]) * (ang_mtrx5[z][j] + ang_mtrx8[z][j]);
            }
            interm2[i][j] = 0;
            for z in 0..aug_size/2 {
                interm2[i][j] += (ang_mtrx3[i][z] + ang_mtrx4[i][z]) * ang_mtrx5[z][j];
            }
            interm3[i][j] = 0;
            for z in 0..aug_size/2 {
                interm3[i][j] += ang_matrix1[i][z] * (ang_mtrx6[z][j] - ang_mtrx8[z][j]);
            }
            interm4[i][j] = 0;
            for z in 0..aug_size/2 {
                interm4[i][j] += ang_mtrx4[i][z] * (ang_mtrx7[z][j] - ang_mtrx5[z][j]);
            }
            interm5[i][j] = 0;
            for z in 0..aug_size/2 {
                interm5[i][j] += (ang_matrix1[i][z] + ang_matrix2[i][z]) * ang_mtrx8[z][j];
            }
            interm6[i][j] = 0;
            for z in 0..aug_size/2 {
                interm6[i][j] += (ang_mtrx3[i][z] - ang_matrix1[i][z]) * (ang_mtrx5[z][j] + ang_mtrx6[z][j]);
            }
            interm7[i][j] = 0;
            for z in 0..aug_size/2 {
                interm7[i][j] += (ang_matrix2[i][z] - ang_mtrx4[i][z]) * (ang_mtrx7[z][j] + ang_mtrx8[z][j]);
            }
        }
    }
    ///////////////////////////////////////////////////////////////////////////////
	///////////////////////Создание вспомогательных матриц/////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut helper1 = vec![vec![0i32; aug_size]; aug_size];
    let mut helper2 = vec![vec![0i32; aug_size]; aug_size];
    let mut helper3 = vec![vec![0i32; aug_size]; aug_size];
    let mut helper4 = vec![vec![0i32; aug_size]; aug_size];

    ///////////////////////////////////////////////////////////////////////////////
	////////////Подсчет значений вспомогательных матриц из промежуточных///////////
	///////////////////////////////////////////////////////////////////////////////
    
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
          helper1[i][j] = interm1[i][j] + interm4[i][j] - interm5[i][j] + interm7[i][j];
          helper2[i][j] = interm3[i][j] + interm5[i][j];
          helper3[i][j] = interm2[i][j] + interm4[i][j];
          helper4[i][j] = interm1[i][j] - interm2[i][j] + interm3[i][j] + interm6[i][j];
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
	///////////////////Создание результирующей матрицы/////////////////////////////
	///////////////////////////////////////////////////////////////////////////////

    let mut aug_res_mtrx = vec![vec![0i32; aug_size]; aug_size];

    ///////////////////////////////////////////////////////////////////////////////
	///////Занесение информации из вспомогательных матриц в результирующую/////////
	///////////////////////////////////////////////////////////////////////////////
    
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            aug_res_mtrx[i][j] = helper1[i][j];
            aug_res_mtrx[i][j + aug_size/2] = helper2[i][j];
            aug_res_mtrx[i + aug_size/2][j] = helper3[i][j];
            aug_res_mtrx[i + aug_size/2][j + aug_size/2] = helper4[i][j];
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
	////////////////Выравнивание границ результирующей матрицы/////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut row = 0;
    let mut col = 0;
    let mut num_rows;
    let mut num_cols;
    for i in 0..aug_size {
        num_rows = 0;
        num_cols = 0;
        for j in 0..aug_size {
            if aug_res_mtrx[i][j] != 0 {
                num_rows += 1;
                row = aug_size;
            }
            if aug_res_mtrx[j][i] != 0 {
                num_cols += 1;
                col = aug_size;
            }
        }
        if num_rows == 0 && i < row {
            row = i;
        }
        if num_cols == 0 && i < col {
            col = i;
        }
    }
    
    let mut res_mtrx = vec![vec![0i32; col]; row];
    
    let rows = if aug_res_mtrx.len() > res_mtrx.len() {
        res_mtrx.len()
    } else {
        aug_res_mtrx.len()
    };
    let cols = if aug_res_mtrx[0].len() > res_mtrx[0].len() {
        res_mtrx[0].len()
    } else {
        aug_res_mtrx[0].len()
    };
    
    for i in 0..rows {
        for j in 0..cols {
            res_mtrx[i][j] = aug_res_mtrx[i][j];
        }
    }

    println!("Результирующая матрица");
    for row in res_mtrx {
        for el in row {
          print!("{} ", el)
        }
        print!("\n");
    }
}

use std::io;

mod functions;
use functions::*;



fn main() {
    println!("{}", usize::min_value());
    println!("Вас приветствует программа");
    println!("Быстрого перемножения матриц методом Штрассена\n");
    ///////////////////////////////////////////////////////////////////////////////
	////////////////////Ввод размеров матрицы пользователем////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    println!("Введите размеры первой матрицы\t");
    let (m1_rows, m1_colls) = enter_size();
    println!("Введите размеры второй матрицы\t");
    let (m2_rows, m2_colls) = enter_size();

    let mut matrix1 = matrix(m1_rows, m1_colls);
    let mut matrix2 = matrix(m2_rows, m2_colls);
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
            enter_matrix(&mut matrix1);
            enter_matrix(&mut matrix2);
            
            println!("\nМатрица 1\n");
            print_matrix(&matrix1);
            println!();
            println!("\nМатрица 2\n");
            print_matrix(&matrix2);

        }
        2 => {
            enter_matrix_auto(&mut matrix1);
            enter_matrix_auto(&mut matrix2);

            println!("\nМатрица 1\n");
            print_matrix(&matrix1);
            println!("\nМатрица 2\n");
            print_matrix(&matrix2);

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
    let mut aug_matrix1 = matrix(aug_size, aug_size);
    let mut aug_matrix2 = matrix(aug_size, aug_size);

    part_matrix(&matrix1, &mut aug_matrix1);
    part_matrix(&matrix2, &mut aug_matrix2);

    println!("Приведенные матрицы");

    println!("\nМатрица 1\n");
    print_matrix(&aug_matrix1);

    println!("\nМатрица 2\n");
    print_matrix(&aug_matrix2);


    ///////////////////////////////////////////////////////////////////////////////
	///////////////Разбиение матриц на подматрицы и их заполнение//////////////////
	///////////////////////////////////////////////////////////////////////////////
    let matrix1_angs = matrix_angs(&aug_matrix1);
    let matrix2_angs = matrix_angs(&aug_matrix2);
	///////////////////////////////////////////////////////////////////////////////
	////////////////////////Создание промежуточных матриц//////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut interm1 = matrix(aug_size, aug_size);
    let mut interm2 = matrix(aug_size, aug_size);
    let mut interm3 = matrix(aug_size, aug_size);
    let mut interm4 = matrix(aug_size, aug_size);
    let mut interm5 = matrix(aug_size, aug_size);
    let mut interm6 = matrix(aug_size, aug_size);
    let mut interm7 = matrix(aug_size, aug_size);

    ///////////////////////////////////////////////////////////////////////////////
	////////////////////Вычисление значений промежуточных матриц///////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    for i in 0..aug_size/2 {
        for j in 0..aug_size/2 {
            for z in 0..aug_size/2 {
                interm1[i][j] += (matrix1_angs[0][i][z] + matrix1_angs[3][i][z]) * (matrix2_angs[0][z][j] + matrix2_angs[3][z][j]);
                interm2[i][j] += (matrix1_angs[2][i][z] + matrix1_angs[3][i][z]) * matrix2_angs[0][z][j];
                interm3[i][j] += matrix1_angs[0][i][z] * (matrix2_angs[1][z][j] - matrix2_angs[3][z][j]);
                interm4[i][j] += matrix1_angs[3][i][z] * (matrix2_angs[2][z][j] - matrix2_angs[0][z][j]);
                interm5[i][j] += (matrix1_angs[0][i][z] + matrix1_angs[1][i][z]) * matrix2_angs[3][z][j];
                interm6[i][j] += (matrix1_angs[2][i][z] - matrix1_angs[0][i][z]) * (matrix2_angs[0][z][j] + matrix2_angs[1][z][j]);
                interm7[i][j] += (matrix1_angs[1][i][z] - matrix1_angs[3][i][z]) * (matrix2_angs[2][z][j] + matrix2_angs[3][z][j]);
            }
        }
    }
    ///////////////////////////////////////////////////////////////////////////////
	///////////////////////Создание вспомогательных матриц/////////////////////////
	///////////////////////////////////////////////////////////////////////////////
    
    let mut helper1 = matrix(aug_size, aug_size);
    let mut helper2 = matrix(aug_size, aug_size);
    let mut helper3 = matrix(aug_size, aug_size);
    let mut helper4 = matrix(aug_size, aug_size);

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

    let mut aug_res_mtrx = matrix(aug_size, aug_size);

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
    
    let mut res_mtrx = matrix(row, col);
    
    part_matrix(&aug_res_mtrx, &mut res_mtrx);
    
    println!("Результирующая матрица");
    print_matrix(&res_mtrx);
}

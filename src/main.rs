use std::env;

fn main() {
    let amounts: (i16, i8, i8) = gather_intel();
    let mut matric: Vec<Vec<i128>>;
    matric = generate_random(amounts.0, amounts.1, amounts.2);
    matric = create_p(matric, amounts.0, amounts.1, amounts.2);
    let print_matrix: Vec<String> = generate_print(matric, amounts.0, amounts.1, amounts.2);
    print_mx(print_matrix);
}

fn gather_intel() -> (i16, i8, i8) {
    let amount_variables: i16;
    let amount_digits: i8;
    let amount_rows: i8;

    let args: Vec<String> = env::args().collect();

    if !args.is_empty() && args.len() == 4 {
        // Get args from command
        amount_variables = args[1].parse::<i16>().unwrap();
        amount_digits = args[2].parse::<i8>().unwrap();
        amount_rows = args[3].parse::<i8>().unwrap();
        (amount_variables, amount_digits, amount_rows)
    } else {
        println!("How many Variables?");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        amount_variables = line.trim().parse::<i16>().unwrap();
        println!("How many Digits should the Variabels have?");
        line = String::new();
        std::io::stdin().read_line(&mut line);
        amount_digits = line.trim().parse::<i8>().unwrap();
        line = String::new();
        println!("How many Rows should the Matrics have?");
        std::io::stdin().read_line(&mut line);
        amount_rows = line.trim().parse::<i8>().unwrap();
        (amount_variables, amount_digits, amount_rows)
    }
}
// generates randoms for matrcis for example in ax+bx+cx+dc = p it generates first all the x, which
// lie in row 0, thereafter it makes all the a,b,c,d from row 1 till row n, row n being
// amount_digits
fn generate_random(amount_variables: i16, amount_digits: i8, amount_rows: i8) -> Vec<Vec<i128>> {
    let mut matric: Vec<Vec<i128>> = Vec::new();
    for _ in 0..amount_rows + 1 {
        let mut row: Vec<i128> = Vec::new();
        for _ in 0..amount_variables {
            row.push(
                ((rand::random::<f32>() - 0.5) * (10_i128.pow(amount_digits as u32)) as f32).round()
                    as i128,
            );
        }
        matric.push(row as Vec<i128>);
    }
    matric
}
// same as in he ax+bx+cx+dx = p example this function generates p by computing the function
// and places the result in the last column of each row
fn create_p(
    mut matrics: Vec<Vec<i128>>,
    amount_variables: i16,
    _amount_digits: i8,
    amount_rows: i8,
) -> Vec<Vec<i128>> {
    for i in 1..amount_rows + 1 {
        let mut r: i128 = 0;
        for b in 0..amount_variables {
            r += matrics[i as usize][b as usize] * matrics[0][b as usize];
        }
        matrics[i as usize].push(r);
    }
    matrics
}
fn generate_print(
    matrix: Vec<Vec<i128>>,
    amount_variables: i16,
    _amount_digits: i8,
    amount_rows: i8,
) -> Vec<String> {
    let mut print: Vec<String> = Vec::new();
    let mut wolfram_alpha_matrix_notation: String = String::from("WolframAlpha Matrix Notation: {");
    for i in 1..amount_rows + 1 {
        let mut line: String = String::from("[");
        wolfram_alpha_matrix_notation += "{";
        for r in 0..(amount_variables + 1) {
            wolfram_alpha_matrix_notation += &format!(
                "{}{}",
                matrix[i as usize][r as usize],
                if r != amount_variables { "," } else { "" }
            );
            line += &format!(
                " {} {}",
                if r == amount_variables { "|" } else { "" },
                matrix[i as usize][r as usize]
            );
        }
        wolfram_alpha_matrix_notation += if i != amount_rows { "}," } else { "}" };
        line += "]";
        print.push(line);
    }
    wolfram_alpha_matrix_notation += "}";
    print.push(wolfram_alpha_matrix_notation);
    print
}
fn print_mx(print_matrix: Vec<String>) {
    for i in print_matrix {
        println!("{}", i);
    }
}

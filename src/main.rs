use std::io;

fn main() {
    println!("Calculadora");

    let mut res: f32 = 0.0;
    loop {
        let mut number_one = String::new();
        let mut operator = String::new();

        println!("Valor: {}", res);
        io::stdin().read_line(&mut number_one).expect("Fail");
        let number_one: f32 = number_one.trim().parse().expect("type a number");

        println!("Escolha o operador");
        io::stdin().read_line(&mut operator).expect("Fail");
        let operator: &str = &operator[..].trim();

        let prev_res = res;
        match operator {
            "+" => {
                res += number_one;
            }
            "-" => {
                res -= number_one;
            }
            "*" => {
                res *= number_one;
            }
            "/" => {
                res /= number_one;
            }
            "c" => {
                println!("Limpando valores");
                res = 0.0;
            }
            _ => {
                println!("Saindo da calculadora");
                break
            },
        }
        println!("{} {} {} = {}", &prev_res, operator, &number_one, res);
    }
}

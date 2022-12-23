fn main() {
    // Casting
    //let nota1 = 8.5f32;

    manipulando_arrays();
    matriz();

    // É fácil passar um valor inválido como parâmetro nesse caso. Não havia uma representação significativa do que é um dia da semana.
    //println!("Fim de semana: {}", eh_fim_de_semana(3));

    // Para resolver esse problema, podemos usar um enum.
    println!("Fim de semana: {}", eh_fim_de_semana(DiaDaSemana::Quarta));
    cores();
}

fn manipulando_arrays() {
    let notas: [f32; 4] = [8.5, 9.0, 7.0, 6.0];

    // Cria um array com 4 posições e preenche com o valor 8.5
    // let notas : [f32; 4] = [8.5; 4];

    /*
    Para acessar endereço de memória de um array, como é o caso do indice 0, usamos a sintaxe abaixo com usize
    ou seja, 4 bytes em sistemas 32 bits e 8 bytes em sistemas 64 bits.
    usize e isize possuem o tamanho necessário para armazenar um endereço de memória,

    Para garantir que conseguimos representar todos os índices possíveis de um array em memória,
    por exemplo, e até para lidarmos com ponteiros, precisamos usar estes tipos.
    */
    let inteiro: usize = 0;

    println!("A nota é: {}", notas[inteiro]);

    for nota in notas {
        println!("Nota: {}", nota);
    }

    for indice in 0..notas.len() {
        println!("A Nota {} é = {}", indice + 1, notas[indice]);
    }
}

fn matriz() {
    println!("Matriz");
    // Com essa definição nós temos um array de 2 linhas e 3 colunas de f32.
    let matriz: [[f32; 3]; 2] = [[1.0, 2.5, 0.3], [4.5, 5.0, 6.9]];

    for linha in matriz {
        for coluna in linha {
            println!("Valor: {}", coluna);
        }
    }
}

fn eh_fim_de_semana(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Sabado | DiaDaSemana::Domingo => true,
        _ => false,
    }
}

fn cores() {
    let c: Color = Color::RgbColor(0, 0, 0);
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    let c: Color = Color::RgbColor(3, 2, 4);

    match c {
        Color::Red => println!("Vermelho"),
        Color::Green => println!("Verde"),
        Color::Blue => println!("Azul"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("Preto"),
        Color::RgbColor(r, g, b) => println!("RGB ({}, {}, {})", r, g, b),
        _ => (), // Quando tratamos todos os casos, o _ não é necessário. Mas se não tratarmos todos os casos, o _ é necessário.
    }
}

// Um enum é um tipo que pode ter um conjunto de valores pré-definidos.
// Os valores de um enum são chamados de variantes.
#[allow(dead_code)]
enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

#[allow(dead_code)] // Para não mostrar o warning
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tupla
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

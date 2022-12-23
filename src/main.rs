fn main() {
    // Casting
    //let nota1 = 8.5f32;

    manipulando_arrays();
    matriz();
}

fn manipulando_arrays() {
    let notas: [f32; 4] = [8.5, 9.0, 7.0, 6.0];

    // Cria um array com 4 posições e preenche com o valor 8.5
    // let notas : [f32; 4] = [8.5; 4];

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

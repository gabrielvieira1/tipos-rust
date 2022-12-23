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
    conteudo_opcional();
    vectors();
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

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from("arquivo.txt"));

    match &conteudo_arquivo {
        Some(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        None => println!("Arquivo não encontrado"),
    };

    // Exibição de debug no Rust é feita com o {:?}
    println!("{:?}", conteudo_arquivo);

    // Prelude do Rust já possui o método unwrap que retorna o valor do Option.
    // Se o valor for None, o unwrap irá gerar um panic.
    // if let é como um match, mas só funciona com Option. Se o valor for Some, ele irá executar o bloco. Se for None, ele irá ignorar.
    // O if let é uma forma de desestruturar o Option.
    // O while let é parecido com o if let, mas ele executa o bloco enquanto o valor for Some.
    // Usar if let é uma outra forma de podermos aplicar pattern matching em rust, além do match statement. https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    if let Some(valor) = conteudo_arquivo {
        println!("Conteúdo do arquivo: {}", valor);
    }

    /*
    while let Some(valor) = conteudo_arquivo {
        println!("Conteúdo do arquivo: {}", valor);
    }
    */

    // O unwrap_or_else é parecido com o unwrap_or, mas recebe uma função como parâmetro.
    /*
    println!(
        "Conteúdo do arquivo: {}",
        conteudo_arquivo.unwrap_or(String::from(""))
    );
    */
}

fn ler_arquivo(nome_arquivo: String) -> Option<String> {
    if nome_arquivo == String::from("arquivo.txt") {
        Some(String::from("Conteúdo do arquivo"))
    } else {
        None
    }
}

fn vectors() {
    //let mut notas: Vec<i32> = Vec::new();

    //let mut notas: Vec<f32> = vec![10.0, 20.0, 30.0];

    // Para evitar ao maximo novas alocações de memória, podemos definir a capacidade inicial do vetor.
    // A função with_capacity define a capacidade inicial do vetor.
    /*
    Alocar memória na heap é um processo custoso, então evitar esse processo é necessário. 
    PS.: Além de alocar a memória, esse processo também demanda a cópia dos dados de um endereço para outro (o que não é tão custoso assim).
    */
    // Alocar um vetor com capacidade inicial de 5 elementos.
    let mut notas: Vec<f32> = Vec::with_capacity(5); 
    notas.push(10.0);
    notas.push(20.0);
    notas.push(30.0);

    println!("Capacidade: {}", notas.capacity());
    println!("Vetor: {:?}", notas);

    // A função push exige que a variável a ser alterada seja mutável (definida com let mut).
    notas.push(40.0);
    notas.push(50.0);
    // A capacidade do vetor é o tamanho do vetor alocado na memória.
    // O tamanho do vetor é o número de elementos que o vetor possui.
    // O vetor é alocado na memória com um tamanho inicial, e quando o vetor atinge o tamanho máximo, ele é duplicado.
    // O vetor é duplicado quando o tamanho máximo é atingido, e não quando o tamanho do vetor é atingido.
    println!("Capacidade: {}", notas.capacity());
    println!("Tamanho do vetor: {}", notas.len());
    println!("Vetor: {:?}", notas);
    println!("Primeira nota: {}", notas[0]);

    // O & é para retornar um ponteiro para o valor, e não o valor em si.
    // O unwrap irá gerar um panic se o valor for None.
    // unwrap_or retorna o valor do Option, ou o valor passado como parâmetro se o valor for None.
    println!("Nota 6 = {}", notas.get(5).unwrap_or(&0.0));
    //println!("Nota 6 = {}", notas.get(5).unwrap());

    // O método get retorna um Option, que pode ser Some ou None.
    // O método get retorna um ponteiro para o valor, e não o valor em si.
    println!(
        "Nota 6 = {}",
        match notas.get(4) {
            Some(nota) => nota,
            None => &0.0,
        }
    );

    /*
    // O método pop remove o último valor do vetor e retorna um Option.
    while let Some(nota) = notas.pop() {
        println!("Valor removido: {}", nota);
    }
     */

    // O método iter retorna um iterador.
    // O iterador é um objeto que pode ser usado para iterar sobre os valores do vetor.
    for nota in notas.iter() {
        println!("Nota no for: {}", nota);
    }

    // O iter_mut retorna um iterador mutável, que pode ser usado para iterar sobre os valores do vetor e alterá-los.
    // O iterador mutável é um objeto que pode ser usado para iterar sobre os valores do vetor e alterá-los.
    for nota in notas.iter_mut() {
        *nota *= 2.0;
    }

    // Para que o Vec seja percorrido, uma função chamada into_iter é chamada recebendo o Vec por parâmetro, 
    // logo, o borrowing precisa ser levado em consideração.
    for nota in &notas {
        println!("Nota no for borrowing: {}", nota);
    }

    println!("Vetor: {:?}", notas);
}

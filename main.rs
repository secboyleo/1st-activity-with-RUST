fn main() {
    println!("Digite o valor do raio > > >");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro em ler buffer");

    let raio: f32 = buffer.trim().parse().expect("erro em ler o raio");
    let pi: f32 = 3.14;

    let area: f32 = pi * raio.powf(2.0);
    print!("O valor da area do circulo é = {}", area);
}
const TIPO_DE_DADO: i8 = 2;
static mut VARIAL_STATICA: i8 = 3;

fn main() {
    unsafe {
        VARIAL_STATICA = 4;
        println!("Constante: {}", TIPO_DE_DADO);
        println!("Variável estática: {}", VARIAL_STATICA);
    }
    funcao();
}

fn funcao() {
    unsafe {
        VARIAL_STATICA = 4;
        println!("Variável estática: {}", VARIAL_STATICA);
    }
}
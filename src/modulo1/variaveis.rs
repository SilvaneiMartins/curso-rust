const TIPO_DE_DADO: i8 = 2;

static mut VARIAVEL_STATICA: i8 = 3;

#[allow(dead_code)]
pub fn funcao() {
    unsafe {
       VARIAVEL_STATICA = 4;

        println!("Constante: {}", TIPO_DE_DADO);
        println!("Variável estática: {}", VARIAVEL_STATICA);
    }
}

#[allow(dead_code, unused_doc_comments)]
pub fn exercicio_1() {
    /**
     * Dado que eu tenha um ano de nascimento, e faço a subtração,
     * Então deve ter o resultado da idade.
     */
    let nome = "Silvanei Martins";

    let ano_nascimento: u16 = 1978;
    let mes_nascimento: u16 = 08;
    let dia_nascimento: u16 = 18;

    let ano_atual: u16 = 2025;
    let mes_autal: u16 = 1;
    let dia_atual: u16 = 6;

    let mut idade = ano_atual - ano_nascimento;

    if mes_nascimento > mes_autal {
        idade -= 1;
    } else if mes_nascimento == mes_autal && dia_nascimento > dia_atual {
        idade -= 1;
    } else {
        idade = idade;
    }

    println!("A idade do ({}) calculado para o ano de {} é de {}!", nome, ano_nascimento, idade);
}

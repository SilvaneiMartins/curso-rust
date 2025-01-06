#[allow(dead_code, unused_doc_comments)]
pub fn exercicio_1() {
    /**
     * Dado que eu tenha um ano de nascimento, e faço a subtração,
     * Então deve ter o resultado da idade.
     */
    let nome = "Silvanei Martins";

    let ano_nascimento: u16 = 1978;
    let mes_nascimento: u16 = 08;
    let ano_atual: u16 = 2025;

    let idade = ano_atual - ano_nascimento;

    println!("A idade do ({}) calculado para o ano de {} é de {}!", nome, ano_nascimento, idade);
}

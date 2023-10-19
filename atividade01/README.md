Nessa atividade, há a implementação da struct `ImgManager`, responsável por salvar imagens em PNG no diretório escolhido pelo usuário.

A _crate_ [image](https://crates.io/crates/image) foi utilizada por permitir a criação, manipulação e armazenamento de imagens dentro da linguagem Rust.

### Organização
- A implementação se encontra em `src/lib.rs`.
- O diretório `imgs` reúne as imagens geradas em testes unitários.

### Testes unitários
A partir do diretório inicial, para executar os testes unitários:
1. Acesse o diretório `atividade01` utilizando o comando `cd atividade01`;
2. Insira o comando `cargo test` para executar os testes.

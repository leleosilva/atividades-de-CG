### Introdução
Nessa atividade, a struct `ImgManager` foi implementada, responsável por salvar imagens em PNG no diretório escolhido pelo usuário.

A _crate_ [image](https://crates.io/crates/image) foi utilizada por permitir a criação, manipulação e armazenamento de imagens dentro da linguagem Rust.

A _crate_ [imageproc](https://crates.io/crates/imageproc) foi utilizada para auxiliar na criação de figuras geométricas nos testes unitários.

- A implementação se encontra em `src/lib.rs`.

### Testes unitários
A partir do diretório inicial, para executar os testes unitários:
1. Acesse o diretório `atividade01` utilizando o comando `cd atividade01`;
2. Insira o comando `cargo test` para executar os testes.

- O diretório `imgs` reúne as imagens geradas pelos testes unitários.

Três testes unitários foram criados:
- `test_create_gradient_img()`: gera uma imagem de gradiente e verifica se foi salva corretamente;
- `test_create_circle_img()`: gera uma imagem com três círculos e verifica se foi salva corretamente;
- `test_create_square_img()`: gera uma imagem com três quadrados e verifica se foi salva corretamente.

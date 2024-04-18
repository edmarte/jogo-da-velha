# Jogo da Velha

Atividade Tópicos Especiais Em Sistema de Informação de Gestão G.

## Sobre
Escrever um jogo da velha em modo texto.

O jogo tem a seguinte espeicifcação:

1. A primeira jogada é do jogador X

2. Mostrar o tabuleiro

3. Ler em qual posição o jogador quer jogar

  a. Posições são numeradas de 1 a 9

  b. Ignorar posições inválidas

4. Atualizar o tabuleiro

5. Se o jogador vencer, mostrar o tabuleiro final e anunciar o fim do jogo

6. Caso contrário, mudar para o próximo jogador

7. Repetir a partir do passo 2

Há dois jogadores: X e O. Um jogador ganha quando completa uma sequência de 3 casas consecutivas iguais, que pode ser horizontal, vertical ou diagnoal.

### Compilação
Para compilar o jogo e executar o código use o comando `cargo run` ou `rustc -o jogodavelha src/main.rs` para compilar e `./jogodavelha` para executar o jogo.

### Implementação
Iniciar partida
  - Ao iniciar a partida o jogo mostra as casas referente a cada número entre 1 e 9.
  - Qualquer jogada fora desse intervalo é considerada jogada invalida.

Vitória
  - A verificação de linhas, colunas e diagonal é feita "na mão" para todas as possibilidades.
  - Como estamos aproveitando o código para verificar tanto o "X" quanto o "O", é preciso adicionar uma condição extra que verifica se o caractere procurado é diferente de _espaço_, o espaço foi usado para inicializar o _array_.
  - Utilizar uma matriz de duas dimenções ajuda na verificação da vitória.

Empate
  - O jogo termina empatado quando não tem nenhum espaço disponivel no tabuleiro.
  - Mesmo que não exista mais possibilidade de vitória, o jogo não termina antes do tabuleiro ser totalmente preenchido.
  - Foi criado um _for_ que verifica se ainda tem o caractere espaço no tabuleiro, se não há mais então o jogo é encerado.
# Código Rust Limpo


## Prefácio: Por que Escrever Código Limpo?

Este documento é uma referência para a comunidade Rust que visa ajudar desenvolvedores a escreverem código mais limpo. Seja trabalhando em um projeto pessoal ou como parte de uma equipe maior, escrever código limpo é uma habilidade importante. Estabelecer bons paradigmas e padrões consistentes e acessíveis para escrever código limpo pode ajudar a prevenir que desenvolvedores percam horas tentando entender seu próprio trabalho (ou o de outros).

"Nós não lemos código, nós o decodificamos" - Peter Seibel

Como desenvolvedores, às vezes somos tentados a escrever código de uma maneira conveniente no momento, sem considerar as melhores práticas; isso torna as revisões de código e testes mais difíceis. De certa forma, estamos "codificando" - e, ao fazer isso, tornando mais difícil para outros decodificarem nosso trabalho. Mas queremos que nosso código seja utilizável, legível e manutenível. E isso requer codificar da maneira *correta*, não da maneira fácil.

Este documento começa com uma introdução simples e curta aos fundamentos da escrita de código limpo. Mais tarde, discutiremos exemplos concretos de refatoração específicos para Rust.

### Sumário
* Introdução ao Código Limpo
   * Desenvolvimento Orientado a Testes
   * Convenções de Nomenclatura
   * Comentários
      * Nomeação de Funções
      * Nomeação de Variáveis
   * Limpando Funções
      * Tamanho de Funções
      * Assinaturas de Funções
   * Escopo de Variáveis
   * Declaração de Variáveis
* Rust Limpo
   * Valores de Retorno
      * Retornando Erros Definidos
      * Retornando Erros Dinâmicos
   * Referências em Rust
   * Closures São Ponteiros de Função
   * Traits em Rust
   * O `dyn Any`
* Resumo

### Introdução ao Código Limpo

Código limpo é o conceito pragmático de promover software legível e manutenível. Código limpo estabelece confiança na base de código e ajuda a minimizar as chances de bugs descuidados serem introduzidos. Também ajuda os desenvolvedores a manter sua agilidade, que normalmente despenca à medida que a base de código se expande devido ao aumento do risco de introduzir bugs.
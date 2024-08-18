# Código Rust Limpo

![comic](assets/clean-code-comic.jpeg)

Adaptado de: [https://github.com/Pungyeon/clean-go-article](https://github.com/Pungyeon/clean-go-article)

## Prefácio: Por que Escrever Código Limpo?

Este documento é uma referência para a comunidade Rust que visa ajudar desenvolvedores a escreverem código mais limpo. Seja trabalhando em um projeto pessoal ou como parte de uma equipe maior, escrever código limpo é uma habilidade importante. Estabelecer bons paradigmas e padrões consistentes e acessíveis para escrever código limpo pode ajudar a prevenir que desenvolvedores percam horas tentando entender seu próprio trabalho (ou o de outros).

*"Nós não lemos código, nós o decodificamos."* – Peter Seibel

Como desenvolvedores, às vezes somos tentados a escrever código de uma maneira conveniente no momento, sem considerar as melhores práticas; isso torna as revisões de código e testes mais difíceis. De certa forma, estamos "codificando" - e, ao fazer isso, tornando mais difícil para outros decodificarem nosso trabalho. Mas queremos que nosso código seja utilizável, legível e manutenível. E isso requer codificar da maneira *correta*, não da maneira fácil.

Este documento começa com uma introdução simples e curta aos fundamentos da escrita de código limpo. Mais tarde, discutiremos exemplos concretos de refatoração específicos para Rust.

### Sumário
1. [Introdução ao Código Limpo](#introdu%C3%A7%C3%A3o-ao-c%C3%B3digo-limpo)
2. [Desenvolvimento Orientado a Testes](#desenvolvimento-orientado-a-testes)
3. [Convenções de Nomenclatura](#conven%C3%A7%C3%B5es-de-nomenclatura)
4. [Comentários](#coment%C3%A1rios)
5. [Nomeação de Funções](#nomea%C3%A7%C3%A3o-de-fun%C3%A7%C3%B5es)
6. [Nomeação de Variáveis](#nomea%C3%A7%C3%A3o-de-vari%C3%A1veis)
7. [Limpeza de Funções](#limpeza-de-fun%C3%A7%C3%B5es)
8. [Comprimento da Função](#comprimento-da-fun%C3%A7%C3%A3o)
9. [Assinaturas de Função](#assinaturas-de-fun%C3%A7%C3%A3o)
10. [Escopo de Variáveis](#escopo-de-vari%C3%A1veis)
11. [Declaração de Variáveis](#declara%C3%A7%C3%A3o-de-vari%C3%A1veis)
12. [Rust Limpo](#rust-limpo)
13. [Valores Retornados](#valores-retornados)
14. [Retornando Erros Definidos](#retornando-erros-definidos)
15. [Retornando Erros Dinâmicos](#retornando-erros-din%C3%A2micos)
16. [Referências em Rust](#refer%C3%AAncias-em-rust)
17. [Fechamentos como Ponteiros de Função](#fechamentos-como-ponteiros-de-fun%C3%A7%C3%A3o)
18. [Traits em Rust](#traits-em-rust)
19. [O `dyn Any`](#o-dyn-any)
20. [Resumo](#resumo)

### Introdução ao Código Limpo

Código limpo é o conceito pragmático de promover software legível e manutenível. Código limpo estabelece confiança na base de código e ajuda a minimizar as chances de bugs descuidados serem introduzidos. Também ajuda os desenvolvedores a manter sua agilidade, que normalmente despenca à medida que a base de código se expande devido ao aumento do risco de introduzir bugs.

### Desenvolvimento Orientado a Testes
O desenvolvimento orientado a testes é a prática de testar seu código frequentemente ao longo de ciclos de desenvolvimento curtos ou sprints. Isso contribui para a limpeza do código ao convidar os desenvolvedores a questionar a funcionalidade e o propósito de seu código. Para facilitar os testes, os desenvolvedores são incentivados a escrever funções curtas que fazem apenas uma coisa. Por exemplo, é consideravelmente mais fácil testar (e entender) uma função de 4 linhas do que uma de 40.

O desenvolvimento orientado a testes consiste no seguinte ciclo:

1. Escrever (ou executar) um teste
2. Se o teste falhar, fazer com que ele passe
3. Refatorar seu código conforme necessário
4. Repetir

Testar e refatorar estão entrelaçados nesse processo. À medida que você refatora seu código para torná-lo mais compreensível ou manutenível, é necessário testar suas mudanças minuciosamente para garantir que você não alterou o comportamento de suas funções. Isso pode ser extremamente útil à medida que a base de código cresce.


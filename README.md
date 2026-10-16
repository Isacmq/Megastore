# ConectaStore — Sistema de Recomendação de Produtos

Esse é o projeto ConectaStore, um sistema de recomendação desenvolvido em Rust para o trabalho da faculdade. O objetivo é resolver o problema de recomendações genéricas da MegaStore usando Grafos.

## Como funciona?
O sistema mapeia Clientes e Produtos como vértices (nós) do grafo e as compras como as conexões (arestas). 
Para gerar as recomendações, usei um algoritmo de busca em largura (BFS). Ele funciona assim:
1. Olha o que a Alice comprou.
2. Vê quais outros clientes (como o Bob) também compraram esse mesmo produto.
3. Descobre o que esses outros clientes compraram e recomenda para a Alice, tirando os itens que ela já tem para não ter duplicados.

## Estruturas de dados usadas
- **Lista de Adjacência com HashMap:** Escolhida por ser muito mais econômica em memória do que uma matriz de adjacência, já que a maioria dos clientes compra só uma fração do catálogo.
- **HashMap para os Vértices:** Permite buscar qualquer produto ou cliente direto pelo ID de forma instantânea.
- **HashSet:** Usado na BFS para controlar os nós já visitados e garantir que o sistema não recomende um produto repetido.

## Como rodar o projeto

Para rodar o sistema e ver a recomendação funcionando no terminal:
```bash
cargo run
```

Para rodar os testes unitários que validam a lógica da recomendação:
```bash
cargo test
```
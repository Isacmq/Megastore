#[allow(dead_code)]
mod graph; 

use graph::{ConectaGraph, TipoVertice, TipoAresta};

fn main() {
    println!("--- Inicializando o Sistema ConectaStore ---");

    let mut grafo = ConectaGraph::new();

    // IDs dos Clientes: 1, 2
    grafo.adicionar_vertice(1, TipoVertice::Cliente { id: 1, nome: "Alice".to_string() });
    grafo.adicionar_vertice(2, TipoVertice::Cliente { id: 2, nome: "Bob".to_string() });

    // IDs dos Produtos: 101, 102, 103
    grafo.adicionar_vertice(101, TipoVertice::Produto { id: 101, nome: "Notebook Gamer".to_string(), preco: "R$ 5000".to_string() });
    grafo.adicionar_vertice(102, TipoVertice::Produto { id: 102, nome: "Mouse Sem Fio".to_string(), preco: "R$ 150".to_string() });
    grafo.adicionar_vertice(103, TipoVertice::Produto { id: 103, nome: "Teclado Mecânico".to_string(), preco: "R$ 350".to_string() });

    // Alice comprou o Notebook (Ida e Volta)
    grafo.adicionar_aresta(1, 101, TipoAresta::Comprou, 1.0);
    grafo.adicionar_aresta(101, 1, TipoAresta::Comprou, 1.0);

    // Alice comprou o Mouse (Ida e Volta)
    grafo.adicionar_aresta(1, 102, TipoAresta::Comprou, 1.0);
    grafo.adicionar_aresta(102, 1, TipoAresta::Comprou, 1.0);

    // Bob comprou o Notebook (Ida e Volta)
    grafo.adicionar_aresta(2, 101, TipoAresta::Comprou, 1.0);
    grafo.adicionar_aresta(101, 2, TipoAresta::Comprou, 1.0);

    // Bob comprou o Teclado Mecânico (Ida e Volta)
    grafo.adicionar_aresta(2, 103, TipoAresta::Comprou, 1.0);
    grafo.adicionar_aresta(103, 2, TipoAresta::Comprou, 1.0);

    println!("\nGerando recomendações para a cliente Alice (ID: 1)...");
    let recomendacoes = grafo.recomendar_produtos(1);

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada no momento.");
    } else {
        println!("Produtos Recomendados de forma personalizada:");
        for produto_id in recomendacoes {
            if let Some(TipoVertice::Produto { nome, preco, .. }) = grafo.vertices.get(&produto_id) {
                println!("-> {} ({})", nome, preco);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_recomendacao_individual() {
        let mut grafo = ConectaGraph::new();
        
        // Cadastra Alice e Bob
        grafo.adicionar_vertice(1, TipoVertice::Cliente { id: 1, nome: "Alice".to_string() });
        grafo.adicionar_vertice(2, TipoVertice::Cliente { id: 2, nome: "Bob".to_string() });
        
        // Cadastra produtos
        grafo.adicionar_vertice(101, TipoVertice::Produto { id: 101, nome: "Notebook".to_string(), preco: "100".to_string() });
        grafo.adicionar_vertice(103, TipoVertice::Produto { id: 103, nome: "Teclado".to_string(), preco: "50".to_string() });

        // Conexões
        grafo.adicionar_aresta(1, 101, TipoAresta::Comprou, 1.0);
        grafo.adicionar_aresta(101, 1, TipoAresta::Comprou, 1.0);
        grafo.adicionar_aresta(2, 101, TipoAresta::Comprou, 1.0);
        grafo.adicionar_aresta(101, 2, TipoAresta::Comprou, 1.0);
        grafo.adicionar_aresta(2, 103, TipoAresta::Comprou, 1.0);
        grafo.adicionar_aresta(103, 2, TipoAresta::Comprou, 1.0);

        let recs = grafo.recomendar_produtos(1);
        
        assert!(recs.contains(&103));
        assert!(!recs.contains(&101));
    }
}

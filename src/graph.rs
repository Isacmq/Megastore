#[allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoVertice {
    Cliente { id: u32, nome: String },
    Produto { id: u32, nome: String, preco: String },
    Categoria { id: u32, nome: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TipoAresta {
    Comprou,
    Avaliou { nota: u8 },
    PertenceACategoria,
    Interesse,
}

#[derive(Debug, Clone)]
pub struct Aresta {
    pub destino: u32,
    pub tipo: TipoAresta,
    pub peso: f32,
}

pub struct ConectaGraph {
    pub vertices: HashMap<u32, TipoVertice>,
    pub adjacencias: HashMap<u32, Vec<Aresta>>,
}

impl ConectaGraph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacencias: HashMap::new(),
        }
    }

    pub fn adicionar_vertice(&mut self, id: u32, vertice: TipoVertice) {
        self.vertices.insert(id, vertice);
        self.adjacencias.entry(id).or_insert(Vec::new());
    }

    pub fn adicionar_aresta(&mut self, origem: u32, destino: u32, tipo: TipoAresta, peso: f32) {
        if self.vertices.contains_key(&origem) && self.vertices.contains_key(&destino) {
            let aresta = Aresta { destino, tipo, peso };
            self.adjacencias.entry(origem).or_insert(Vec::new()).push(aresta);
        }
    }

    pub fn recomendar_produtos(&self, cliente_id: u32) -> Vec<u32> {
        use std::collections::{VecDeque, HashSet};

        let mut recomendados = HashSet::new();
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();

        fila.push_back(cliente_id);
        visitados.insert(cliente_id);

        let mut produtos_comprados = HashSet::new();
        if let Some(arestas) = self.adjacencias.get(&cliente_id) {
            for aresta in arestas {
                if let TipoAresta::Comprou = aresta.tipo {
                    produtos_comprados.insert(aresta.destino);
                    if visitados.insert(aresta.destino) {
                        fila.push_back(aresta.destino);
                    }
                }
            }
        }

        while let Some(atual_id) = fila.pop_front() {
            if let Some(arestas) = self.adjacencias.get(&atual_id) {
                for aresta in arestas {
                    if produtos_comprados.contains(&atual_id) {
                        if let TipoAresta::Comprou = aresta.tipo {
                            if visitados.insert(aresta.destino) {
                                fila.push_back(aresta.destino);
                            }
                        }
                    } else {
                        if let TipoAresta::Comprou = aresta.tipo {
                            if !produtos_comprados.contains(&aresta.destino) {
                                recomendados.insert(aresta.destino);
                            }
                        }
                    }
                }
            }
        }

        recomendados.into_iter().collect()
    }
}

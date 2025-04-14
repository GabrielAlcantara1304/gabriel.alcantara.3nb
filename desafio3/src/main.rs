use desafio3::MyGraph;

fn main() {
    let g = MyGraph::new();
    g.export_dot("grafo.dot");
    println!("Arquivo grafo.dot gerado!");
}

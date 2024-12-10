use yew::prelude::*;
use petgraph::dot::Dot;

#[function_component(GraphView)]
pub fn graph_view() -> Html {
    let dot = format!("{:?}", Dot::with_config(&your_graph, &[Config::EdgeNoLabel]));
    html! {
        <div>
            <h1>{"Graph Visualization"}</h1>
            <pre>{ dot }</pre>
        </div>
    }
}
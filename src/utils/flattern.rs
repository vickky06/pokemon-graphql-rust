use crate::routes::evolution::Chain;

pub fn flatten_evolution_chain_iterative(chain: Chain) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut stack = vec![(chain, vec![])];

    while let Some((node, path)) = stack.pop() {
        let mut new_path = path.clone();
        new_path.push(node.species.name.clone());

        if node.evolves_to.is_empty() {
            result.push(new_path);
        } else {
            for child in node.evolves_to.iter().rev() {
                stack.push(((*child).clone(), new_path.clone()));
            }
        }
    }

    result
}

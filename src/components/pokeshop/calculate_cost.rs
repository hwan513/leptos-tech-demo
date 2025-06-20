/// Calculate the total cost of the cart
pub fn calculate_cost(cart: Vec<usize>, costs: &[usize]) -> usize {
    let mut cost = 0;
    for val in cart {
        cost += costs[val];
    }
    cost
}

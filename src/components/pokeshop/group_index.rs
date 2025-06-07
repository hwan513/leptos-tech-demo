/// Returns a vector of the number of items in each group within the cart
pub fn group_index(cart: Vec<usize>) -> Vec<usize> {
    let mut groups = vec![0; 6];
    for id in cart {
        groups[id] += 1;
    }
    groups
}

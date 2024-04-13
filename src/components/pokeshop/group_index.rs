pub fn group_index(cart: Vec<usize>) -> Vec<usize> {
    // Using a vec to be lazy here. Better way is to use a hashmap
    let mut groups = vec![0; 6];
    for id in cart {
        groups[id] += 1;
    }
    groups
}

pub fn ids_difference(first_ids: Vec<i32>, second_ids: Vec<i32>) -> Vec<i32> {
    first_ids
        .into_iter()
        .filter(|&id| !second_ids.contains(&id))
        .collect()
}

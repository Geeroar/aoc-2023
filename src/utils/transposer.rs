#![allow(dead_code)]

pub fn transpose_vec_of_strings(vec_of_string: Vec<String>) -> Vec<String> {
    /*
       Handy if you want to read a list of string top to bottom,
       but then want to read it left to right.

       e.g.

       [
           "*##",
           "#**",
           "**#"
       ]

       becomes

       [
           "*#*",
           "#**",
           "#*#"
       ]
    */
    let matrix: Vec<Vec<char>> = vec_of_string.iter().map(|s| s.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new 2D vector for the transposed matrix
    let mut transposed_matrix = vec![vec![' '; rows]; cols];

    // Transpose the matrix
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            transposed_matrix[j][i] = value;
        }
    }

    // Convert the vectors back to strings
    let transposed_strings: Vec<String> = transposed_matrix
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect();

    return transposed_strings;
}

#[cfg(test)]
mod tests {

    use super::transpose_vec_of_strings;

    #[test]
    fn test_transposing() {
        let input: Vec<String> = vec!["*##", "#**", "**#"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let expected: Vec<String> = vec!["*#*", "#**", "#*#"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = transpose_vec_of_strings(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_transpose() {
        let input: Vec<String> = vec!["*##", "#**", "**#"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = transpose_vec_of_strings(transpose_vec_of_strings(input.clone()));
        assert_eq!(result, input);
    }
}

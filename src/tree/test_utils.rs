#[cfg(test)]
pub fn assert_canonical_eq(left: &str, right: &str) {
    // Remove extra leadings
    let left_rows = remove_leading(left);
    let right_rows = remove_leading(right);

    assert_eq!(
        left_rows,
        right_rows,
        "\nLeft:\n{}\nRight:\n{}\n",
        left_rows.join("\n"),
        right_rows.join("\n")
    );
}

#[cfg(test)]
fn remove_leading(input: &str) -> Vec<&str> {
    // Filter out the empty lines
    let rows: Vec<&str> = input
        .split('\n')
        .filter(|line| line.trim().len() != 0)
        .collect();

    let extra_leading = rows
        .iter()
        .map(|row| row.chars().position(|c| c != ' ').unwrap_or(0))
        .min()
        .unwrap_or(0);

    rows.iter().map(|row| &row[extra_leading..]).collect()
}

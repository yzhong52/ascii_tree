// Example output:
// ```
// Ccomparison:
//     0: ├─ Root 1 | ├─ Root 1
//     1: └─ Root 2 | │  ├─ Child 1 👈👈👈 differ from row: 1
//     2:           | │  └─ Child 2
//     3:           | └─ Root 2
// Left:
// ├─ Root 1
// └─ Root 2
// Right:
// ├─ Root 1
// │  ├─ Child 1
// │  └─ Child 2
// └─ Root 2
// ```
#[cfg(test)]
pub fn assert_canonical_eq(left: &str, right: &str) {
    // Remove extra leadings
    let left_rows = remove_leading(left);
    let right_rows = remove_leading(right);

    let max_left_width = left_rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);
    let max_right_width = right_rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);

    let mut differ_from_row: Option<usize> = None;
    let mut row_by_row_comparison = vec![];
    for row_idx in 0..std::cmp::max(left_rows.len(), right_rows.len()) {
        let left_row_with_padding = pad_to_len(&left_rows, row_idx, max_left_width);
        let right_row_with_padding = pad_to_len(&right_rows, row_idx, max_right_width);

        let comparison_result: String;
        if left_row_with_padding.trim() != right_row_with_padding.trim()
            && differ_from_row.is_none()
        {
            differ_from_row = Some(row_idx);
            comparison_result = format!(
                "{:5}: {} | {} 👈👈👈 differ from row: {}",
                row_idx, left_row_with_padding, right_row_with_padding, row_idx
            )
        } else {
            comparison_result = format!(
                "{:5}: {} | {}",
                row_idx, left_row_with_padding, right_row_with_padding
            )
        }
        row_by_row_comparison.push(comparison_result);
    }

    assert!(
        differ_from_row.is_none(),
        "\nComparison:\n{}\nLeft:\n{}\nRight:\n{}\n",
        row_by_row_comparison.join("\n"),
        left_rows.join("\n"),
        right_rows.join("\n")
    );
}

fn pad_to_len(rows: &Vec<&str>, idx: usize, len: usize) -> String {
    let mut output = if idx < rows.len() {
        rows[idx].to_string()
    } else {
        "".to_string()
    };

    while output.chars().count() < len {
        output += " "
    }
    output
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

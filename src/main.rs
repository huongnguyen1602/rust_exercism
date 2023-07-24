// mod helloworld;
// mod reverse_string;
// mod giga_second;
// mod decimal;
// mod clock;
// mod anagram;
// mod space_age;
// mod sublist;


// fn main() {
//     // helloworld::hello();
//     // reverse_string::test();
//     // giga_second::test();
//     // decimal::test();
//     // clock::test();
//     // anagram::test();
//     // space_age::test();
//     // sublist::test();
// }


fn annotate(minefield: &[&str]) -> Vec<String>{
    return handle_all_rows(minefield)
}

fn handle_all_rows(rows: &[&str]) -> Vec<String>{
    rows.iter().enumerate().map(|(num, row)| handle_row(row, &num, rows)).collect()
}

fn handle_row(row: &str, index: &usize, minefield: &[&str]) -> String{
    row.chars().enumerate().map(|(i, chr)| count_surroundings(minefield, &i, index, &chr)).collect()
}

fn count_surroundings(mine_field: &[&str], x_index: &usize, y_index: &usize, chr: &char) -> String {
    if chr != &'*' {
        let indices = handle_indices(x_index, y_index, &mine_field[0].len(), &mine_field.len());
        let minefield_bytes = mine_field.iter().map(|e| e.as_bytes()).collect::<Vec<&[u8]>>();
        indices.iter().filter(|(x,y)| minefield_bytes[*y][*x] == "*".as_bytes()[0])
        .map(|(_,_)|  1).reduce(|acc, e| acc+e)
        .unwrap_or(0).to_string().replace("0", " ")
    } else if chr == &'*' {
        '*'.to_string()
    } else {
        ' '.to_string()
    }

}


fn handle_indices(x_index: &usize, y_index: &usize, x_size: &usize, y_size: &usize) -> Vec<(usize, usize)>{
    let x_indices = [
        x_index.checked_sub(1),
        x_index.checked_sub(1),
        x_index.checked_sub(1),
        x_index.checked_add(1),
        x_index.checked_add(1),
        x_index.checked_add(1),
        x_index.checked_add(0),
        x_index.checked_add(0),
    ];
    let y_indices = [
        y_index.checked_add(1),
        y_index.checked_add(0),
        y_index.checked_sub(1),
        y_index.checked_add(1),
        y_index.checked_add(0),
        y_index.checked_sub(1),
        y_index.checked_add(1),
        y_index.checked_sub(1),
    ];
    x_indices.iter().zip(y_indices.iter()).filter(
        |(x,y)| x.unwrap_or(usize::MAX) != usize::MAX && y.unwrap_or(usize::MAX) != usize::MAX
    )
    .map(|(x,y)| (x.unwrap(), y.unwrap()))
    .collect::<Vec<_>>()
}
fn main() {
    // test()


}




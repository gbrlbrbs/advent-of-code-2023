use aoc::day3::sum_of_parts;

#[test]
fn page_test() {
    let data: String = String::from(
        "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....");
    let input: Vec<String> = data.split("\n").map(String::from).collect();
    let sum_parts = sum_of_parts(&input);
    assert_eq!(sum_parts, 2344);
}
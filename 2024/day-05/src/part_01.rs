pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let (rules, updates) = transform_data(data);
    let update_list = transform_updates(&updates);

    let sanitized_updates: Vec<&Update> = update_list.iter().filter(|update| update.iter().all(|printing_order| printing_order.validate(&rules))).collect();
    let sanitized_extract: Vec<&i32> = sanitized_updates.iter().map(|v| v[v.len()/2].page).collect();

    let final_result = sanitized_extract.iter().fold(0, |sum, x| sum + **x);

    println!("Part 1 final result: {}", final_result);
}

fn transform_data(data: Vec<String>) -> (Vec<PageOrdering>, Vec<Vec<i32>>) {
    let mut page_ordering = Vec::new();
    let mut update_list = Vec::new();

    for line in data {
        if line.is_empty() { continue }

        if line.contains("|") {
            /* Page ordering */
            let order = parse_pipe_number_list(&line);
            page_ordering.push(PageOrdering { first: order[0], second: order[1] });
        } else {
            /* Update list */
            let updates: Vec<i32> = parse_comma_number_list(&line);
            update_list.push(updates);
        }
    }

    (page_ordering, update_list)
}

/* Input string examples:
    [41,48,83,86,17]

    Result: Vec of numbers
*/
fn parse_comma_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    parse_number_list(s, ",")
}
fn parse_pipe_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    parse_number_list(s, "|")
}

fn parse_number_list<T: std::str::FromStr>(s: &str, sep: &str) -> Vec<T> {
    s.split(sep).filter_map(|s| s.parse::<T>().ok()).collect()
}


#[derive(Debug)]
struct PageOrdering {
    first: i32,
    second: i32,
}
#[derive(Debug, Copy, Clone)]
struct PrintingOrder<'a> {
    page: &'a i32,
    printed_before: &'a [i32],
    printed_after: &'a [i32],
}
impl PrintingOrder<'_> {
    fn validate(self, rules: &Vec<PageOrdering>) -> bool {
        let before_is_valid = self.printed_before.iter().all(|printed_page| rules.iter().any(|order| &(order.first) == printed_page && &(order.second) == self.page));
        let after_is_valid = self.printed_after.iter().all(|page_to_print| rules.iter().any(|order| &(order.first) == self.page && &(order.second) == page_to_print));

        before_is_valid && after_is_valid
    }
}
type Update<'a> = Vec<PrintingOrder<'a>>;

fn transform_updates(list: &Vec<Vec<i32>>) -> Vec<Update> {
        list.iter().map(|l| get_update(&l)).collect()
}
fn get_update(list: &Vec<i32>) -> Update {
    list.iter().enumerate().map(|(index, _)| PrintingOrder { page: &list[index], printed_before: &(list[0..index]), printed_after: &(list[index+1..])}).collect()
}

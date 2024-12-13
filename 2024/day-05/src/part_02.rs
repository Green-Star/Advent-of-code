use std::cmp::Ordering;

pub fn resolve(input_data_path: &str) {
    /* Load data */
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let (rules, updates) = transform_data(data);

    /* Compute the Updates made */
    /*  (one list of Update for each list of printed pages in `updates`) */
    let update_list = transform_updates(&updates);

    /* Find the wrong updates list: */
    /*  for each Update, if at least one of its printed pages is NOT valid (i.e. it doesn't match the printing rules) then this Update is wrong */

    /*  First, we get the indexes of all the wrong update list in the original vector of update lists  */
    let wrong_updates_indexes: Vec<usize> = update_list.iter().enumerate().filter(|(_, update)| update.iter().any(|printing_order| printing_order.validate(&rules) == false)).map(|(index, _)| index).collect();
    /* And then, we clone each list of printed pages */
    /* Since neither the `updates` (the list of update list loaded from the input data) nor the `update_list` (the list of Update computed from the update list)
        vectors have been sorted, we have a 1:1 relationship
        (i.e. `update_list[index]` is the list of Update computed from the list of printed pages at `updates[index]`)
     */
    let wrong_updates: Vec<Vec<i32>> = wrong_updates_indexes.iter().map(|index| updates[*index].clone()).collect();

    println!("{:?}", wrong_updates);

    println!("*****");

    /* Fix each falsy-update (i.e. reorder the list of printed pages according to the rules) */
    let fixed_updates: Vec<Vec<i32>> = wrong_updates.into_iter().map(|wrong_update| fix_update(wrong_update, &rules)).collect();
    println!("{:?}", fixed_updates);

    /* And then get all middle page numbers of these now valid updates */
    let sanitized_extract: Vec<i32> = fixed_updates.iter().map(|v| v[v.len()/2]).collect();
    /* And sum it to get the final result */
    let final_result = sanitized_extract.iter().fold(0, |sum, x| sum + *x);

    println!("Part 2 final result: {}", final_result);
}

/* Extract both the printing rules and the update lists from the (lodaed) input file */
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
    /* Check if one printed page is valid according the printing rules `rules`
        A printed page `P` is valid if
            all pages in its printed_before list (B) have a matching printing rule B|P (i.e. a rule where B must be printed before P)
            AND
            all pages in its printed_after list (A) have a matching printing rule P|A (i.e. a rule where P must be printed before A)
    */
    fn validate(self, rules: &Vec<PageOrdering>) -> bool {
        let before_is_valid = self.printed_before.iter().all(|printed_page| rules.iter().any(|order| &(order.first) == printed_page && &(order.second) == self.page));
        let after_is_valid = self.printed_after.iter().all(|page_to_print| rules.iter().any(|order| &(order.first) == self.page && &(order.second) == page_to_print));

        before_is_valid && after_is_valid
    }
}
type Update<'a> = Vec<PrintingOrder<'a>>;

/* Create a list of Update from a list of printed pages */
fn transform_updates(list: &Vec<Vec<i32>>) -> Vec<Update> {
        list.iter().map(|l| get_update(&l)).collect()
}
/* Create an Update from an update list
    An update list is a vector on int
    An Update is a vector where each element has 3 attributes:
        . page: the page number of the corresponding integer in the vector list
        . printed_before: the list (possibly empty) of pages printed before this element
        . printed_after: the list (possibly empty) of pages printed after this element
*/
fn get_update(list: &Vec<i32>) -> Update {
    list.iter().enumerate().map(|(index, _)| PrintingOrder { page: &list[index], printed_before: &(list[0..index]), printed_after: &(list[index+1..])}).collect()
}

/* Simply re-order the update list according to the rules provided */
fn fix_update(mut update: Vec<i32>, rules: &Vec<PageOrdering>) -> Vec<i32> {
    update.sort_by(|a, b| sort_pages(a, b, rules));

    update
}
fn sort_pages(a: &i32, b: &i32, rules: &Vec<PageOrdering>) -> Ordering {
    if rules.iter().any(|rule| &(rule.first) == a && &(rule.second) == b) { return Ordering::Less }
    if rules.iter().any(|rule| &(rule.first) == b && &(rule.second) == a) { return Ordering::Greater }

    println!("No direct rule for ({} | {})!!!", a, b);
    Ordering::Equal
}
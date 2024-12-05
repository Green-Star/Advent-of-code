pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let (page_ordering, update_list) = transform_data(data);

    println!("{:?}", page_ordering);
    println!("{:?}", update_list);

    let final_result = 0;

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

fn main() {
  let test: Vec<i32> = vec![75, 47, 61, 53, 29];

  let more: Update = test.iter().enumerate().map(|(index, _)| PrintingOrder { page: &test[index], printed_before: &(test[0..index]), printed_after: &(test[index+1..])}).collect();

  println!("{:?}", more);

  println!("{:?}", [].iter().all(|_e: &&i32| false));

  println!("*****");

  let test: Vec<i32> = vec![75,29,13];
  let foobar: Update = test.iter().enumerate().map(|(index, _)| PrintingOrder { page: &test[index], printed_before: &(test[0..index]), printed_after: &(test[index+1..]) }).collect();

  let rules = vec![
    PageOrdering { first: 75, second: 29 },
    PageOrdering { first: 75, second: 53 },
    PageOrdering { first: 75, second: 47 },
    PageOrdering { first: 97, second: 75 },
    PageOrdering { first: 75, second: 61 },
    PageOrdering { first: 29, second: 13 },
    PageOrdering { first: 97, second: 29 },
    PageOrdering { first: 53, second: 29 },
    PageOrdering { first: 61, second: 29 },
    PageOrdering { first: 47, second: 29 },
    PageOrdering { first: 75, second: 13 },
    PageOrdering { first: 53, second: 13 },
    PageOrdering { first: 61, second: 13 },
    PageOrdering { first: 47, second: 13 },
  ];

  println!("{:?}", foobar);
  println!("{:?}", foobar[0].validate(&rules));
  println!("{:?}", foobar[1].validate(&rules));
  println!("{:?}", foobar[2].validate(&rules));

  println!("Result: {:?}", foobar.iter().all(|printing_order| printing_order.validate(&rules)));

  let toto: Vec<Update> = vec![more, foobar];
  let sanitize: Vec<&Update> = toto.iter().filter(|update| update.iter().all(|printing_order| printing_order.validate(&rules))).collect();
  println!("------");
  println!("{:?}", sanitize);
  let sanitized_extract: Vec<&i32> = sanitize.iter().map(|v| v[v.len()/2].page).collect();
  println!("{:?}", sanitized_extract);
}





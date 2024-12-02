use std::i32;

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let reports = transform_data(data);

    let final_result = reports.iter().filter(|report| report.is_safe()).collect::<Vec<&Report>>().len();

    println!("Part 1 final result: {}", final_result);
}

#[derive(Debug, Clone)]
struct Report {
    level: Vec<i32>,
}
impl Report {
    fn is_safe(&self) -> bool {
        let is_safe = self.level.windows(2).fold(Ok(0), |previous, slice| {
                                match previous {
                                    Err(e) => Err(e),
                                    Ok(previous_diff) => {
                                        let diff = slice[0] - slice[1];
                                        match previous_diff {
                                            0 => Ok(diff),
                                            i32::MIN..=-1 => if -3 <= diff && diff <= -1 { Ok(diff) } else { Err(()) },
                                            1..=i32::MAX  => if 1 <= diff && diff <= 3 { Ok(diff) } else { Err(()) },
                                        }
                                    }
                                }
                            });
        match is_safe {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

fn transform_data(data: Vec<String>) -> Vec<Report> {
    let mut result = Vec::new();

    for s in data {
        let mut line = Vec::new();
        for i in s.split_whitespace() {
            line.push(i.parse().unwrap());
        }
        result.push(Report { level: line });
    }

    result
}

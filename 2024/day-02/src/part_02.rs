use std::i32;

pub fn resolve(input_data_path: &str) {
    let data = crate::core::load_file_in_memory(input_data_path).unwrap();
    let reports = transform_data(data);

    let safe_reports = reports.iter().filter(|report| report.is_safe()).collect::<Vec<&Report>>();
    let unsafe_reports = reports.iter().filter(|report| report.is_safe() == false).collect::<Vec<&Report>>();

    let cleaned_reports = clean_unsafe_reports(unsafe_reports);

    let final_result = safe_reports.len() + cleaned_reports.len();
    println!("Final result: {} safe reports, {} unsafe reports, {} total", safe_reports.len(), cleaned_reports.len(), final_result);


    println!("Part 2 final result: {}", final_result);
}

#[derive(Debug, Clone)]
struct Report {
    level: Vec<i32>,
}
impl Report {
    fn analyze(&self) -> Result<i32, ()> {
        self.level.windows(2).fold(Ok(0), |previous, slice| {
            match previous {
                Err(e) => Err(e),
                Ok(previous_diff) => {
                    let diff = slice[0] - slice[1];
                    match previous_diff {
                        0 => if 1 <= diff.abs() && diff.abs() <= 3 { Ok(diff) } else { Err(()) },
                        i32::MIN..=-1 => if -3 <= diff && diff <= -1 { Ok(diff) } else { Err(()) },
                        1..=i32::MAX  => if 1 <= diff && diff <= 3 { Ok(diff) } else { Err(()) },
                    }
                }
            }
        })
    }

    fn is_safe(&self) -> bool {
        match self.analyze() {
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

fn clean_unsafe_reports(unsafe_reports: Vec<&Report>) -> Vec<&Report> {
    let mut result = Vec::new();

    for r in unsafe_reports {
        match clean_report(&r) {
            Err(_) => {},
            Ok(_) => result.push(r),
        }
    }

    result
}


fn clean_report(report: &Report) -> Result<&Report, ()> {
    let mut cleaned_report = Vec::new();

    for i in 0..report.level.len() {
        let r = Report { level: [ &report.level[0..i], &report.level[i+1..] ].concat() };
        cleaned_report.push(r);
    }

    for r in cleaned_report {
        if r.is_safe() {
            println!("Found clean report {:?} from {:?}", r, report);
            return Ok(report);
        }
    }

    println!("Report {:?} is not clean", report);
    Err(())
}

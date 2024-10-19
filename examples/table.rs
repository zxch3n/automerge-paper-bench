use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    ops::ControlFlow,
    path::PathBuf,
    time::Duration,
};
use tabled::settings::Style;

pub fn main() {
    use tabled::Table;

    let mut rows = Vec::new();
    let group = "Old Snapshot";
    if let ControlFlow::Break(_) = extract_for_group(group, &mut rows) {
        return;
    }

    let group = "New Snapshot";
    if let ControlFlow::Break(_) = extract_for_group(group, &mut rows) {
        return;
    }

    let group = "Shallow Snapshot";
    if let ControlFlow::Break(_) = extract_for_group(group, &mut rows) {
        return;
    }

    let table = Table::new(rows.clone())
        .with(tabled::settings::merge::Merge::vertical())
        .to_string();
    println!("{}", table);
    println!();
    println!();
    println!();
    println!();

    let md_table = Table::new(rows)
        .with(tabled::settings::merge::Merge::vertical())
        .with(Style::markdown())
        .to_string();
    println!("{}", md_table);
}

fn extract_for_group(group: &str, rows: &mut Vec<TableRow>) -> ControlFlow<()> {
    let criterion_dir = format!("./target/criterion/{}/", group);
    let mut targets = vec![];
    if let Ok(entries) = fs::read_dir(criterion_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let path = entry.path();
                    if let Some(path_str) = path.to_str() {
                        targets.push(path_str.to_string());
                    }
                }
            }
        }
    } else {
        println!("Failed to read criterion directory. You should call `cargo bench` before using this program.");
        return ControlFlow::Break(());
    }
    targets.sort();

    for target in targets {
        let row = gen_benchmark_row(&target, group);
        rows.push(row);
    }
    ControlFlow::Continue(())
}

#[derive(tabled::Tabled, Clone)]
struct TableRow {
    name: String,
    task: String,
    time: String,
}

fn gen_benchmark_row(path_to_criterion_result: &str, name: &str) -> TableRow {
    let mut path = PathBuf::from(path_to_criterion_result);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let task = file_name.to_string();
    path.push("new");
    path.push("estimates.json");
    println!("path: {}", path.to_str().unwrap());
    let file = File::open(path).unwrap();
    let parsed: BenchmarkEstimates = serde_json::from_reader(file).unwrap();
    TableRow {
        name: name.to_string(),
        task,
        time: format!(
            "{:?} +- {:?}",
            Duration::from_nanos(parsed.mean.point_estimate as u64),
            Duration::from_nanos(parsed.mean.standard_error as u64)
        ),
    }
}

#[derive(Serialize, Deserialize)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Serialize, Deserialize)]
struct Estimate {
    confidence_interval: ConfidenceInterval,
    point_estimate: f64,
    standard_error: f64,
}

#[derive(Serialize, Deserialize)]
struct BenchmarkEstimates {
    mean: Estimate,
    median: Estimate,
    median_abs_dev: Estimate,
    #[serde(default)]
    slope: Option<Estimate>,
    std_dev: Estimate,
}

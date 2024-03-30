use clap::Parser;
use std::fmt::Write;
use std::io::{BufRead, BufReader, Write as _};
use std::iter;
use std::process::{Command, Stdio};

#[derive(Parser)]
struct Args {
    #[structopt(short, long)]
    cols: String,

    #[structopt(short, long)]
    rows: String,
}

fn main() {
    let args = Args::parse();

    let cols = parse_array(&args.cols);
    let rows = parse_array(&args.rows);
    let code = compile(&cols, &rows);
    let result = solve(&code, rows.len());

    println!("{}", result.trim_end());
}

fn parse_array(s: &str) -> Vec<Vec<usize>> {
    s.split('|')
        .map(|s| s.split(',').map(|col| col.parse().unwrap()).collect())
        .collect()
}

fn compile(cols: &[Vec<usize>], rows: &[Vec<usize>]) -> String {
    let mut code = String::new();

    for x in 0..cols.len() {
        _ = writeln!(code, "(declare-const col{} String)", x);
        _ = writeln!(code, "(assert (= {} (str.len col{})))", rows.len(), x);
    }

    for y in 0..rows.len() {
        _ = writeln!(code, "(declare-const row{} String)", y);
        _ = writeln!(code, "(assert (= {} (str.len row{})))", cols.len(), y);
    }

    for x in 0..cols.len() {
        for y in 0..rows.len() {
            _ = writeln!(
                code,
                "(assert (= (str.at col{} {}) (str.at row{} {})))",
                x, y, y, x
            );
        }
    }

    for (rule_ty, rules) in [("col", cols), ("row", rows)] {
        for (rule_idx, rules) in rules.iter().enumerate() {
            let space = r#"(re.* (str.to.re " "))"#.to_string();

            let rules = rules.iter().enumerate().map(|(i, n)| {
                let rule = format!(r#"((_ re.loop {} {}) (str.to.re "*"))"#, n, n);

                if i > 0 {
                    format!(r#"(re.++ (re.+ (str.to.re " ")) {})"#, rule)
                } else {
                    rule
                }
            });

            let regex = iter::once(space.clone())
                .chain(rules)
                .chain(iter::once(space))
                .collect::<Vec<_>>()
                .join(" ");

            _ = writeln!(
                code,
                r#"(assert (str.in.re {}{} (re.++ {})))"#,
                rule_ty, rule_idx, regex
            );
        }
    }

    code
}

fn solve(code: &str, rows: usize) -> String {
    let child = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldn't spawn z3");

    let mut stdin = child.stdin.unwrap();
    let stdout = child.stdout.unwrap();

    stdin.write_all(code.as_bytes()).unwrap();
    stdin.write_all(b"(check-sat)\n").unwrap();
    stdin.write_all(b"(get-model)\n").unwrap();

    drop(stdin);

    let lines = BufReader::new(stdout)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    if lines[0].contains("unsat") {
        "unsat".into()
    } else {
        let mut rows = vec![String::default(); rows];

        for (line_idx, line) in lines.iter().enumerate() {
            let Some(row_idx) = line.trim().strip_prefix("(define-fun row") else {
                continue;
            };

            let row_idx: usize = row_idx
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();

            let row = lines[line_idx + 1]
                .trim()
                .strip_prefix('"')
                .unwrap()
                .strip_suffix("\")")
                .unwrap();

            rows[row_idx] = row.into();
        }

        rows.join("\n")
    }
}

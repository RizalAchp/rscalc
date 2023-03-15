use std::{
    env,
    io::{self, BufRead, Read, Write},
};

fn run() -> rscalc::Result<()> {
    eprint!("=> ");
    let mut errs = String::new();
    let mut evals = String::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let mut expr = match line {
            Ok(line) => rscalc::from_str(line),
            Err(err) => {
                let err = err.to_string();
                errs.push_str(&err);
                continue;
            }
        };
        match &mut expr {
            Ok(ex) => match ex.eval() {
                Some(ok) => {
                    let e = ok.to_string();
                    evals.push_str(&e);
                }
                None => continue,
            },
            Err(err) => {
                let err = err.to_string();
                errs.push_str(&err);
            }
        };
        if !evals.is_empty() {
            println!("=> {evals}");
            evals.clear();
        }
        if !errs.is_empty() {
            println!("=> {errs}");
            errs.clear();
        }
        eprint!("\n=> ");
    }
    Ok(())
}

fn main() -> rscalc::Result<()> {
    let args = env::args();
    if args.len() < 2 {
        return run();
    }
    let args: Vec<_> = args.into_iter().collect();
    let joined: Option<String> = args.get(1..).map(|x| x.join(" "));
    if let Some(content) = joined {
        let mut expr = rscalc::from_str(content)?;
        if let Some(val) = expr.eval() {
            eprintln!("{val}")
        }
    }

    Ok(())
}

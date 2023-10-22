use std::error::Error;
use std::fmt;
use std::io;

pub struct MyCsv {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl MyCsv {
    pub fn new(data: String) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<&str> = data.trim().split('\n').collect();
        if lines.is_empty() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "No data found",
            )));
        };

        let headers = lines[0]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        // let mut table = format!("{:<16}", headers.join("\t"));

        let mut rows: Vec<Vec<String>> = vec![];
        for line in lines.iter().skip(1) {
            rows.push(
                line.split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
            // table += &format!("\n{:<16}", values.join("\t"));
        }

        Ok(MyCsv { headers, rows })
    }
}

impl fmt::Display for MyCsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_WIDTH: usize = 16;

        let print_row = |f: &mut fmt::Formatter<'_>, row: &Vec<String>| -> fmt::Result {
            write!(f, "| ")?;
            for cell in row.iter() {
                write!(f, "{:width$} | ", cell, width = MAX_WIDTH)?;
            }
            writeln!(f)
        };
        let total_width = (MAX_WIDTH + 3) * self.headers.len() - 1;
        // print top border
        writeln!(f, "+{:1$}+", "", total_width)?;
        // print header row
        print_row(f, &self.headers)?;
        // rint border under headers
        writeln!(f, "+{:-<1$}+", "", total_width)?;
        // print each data row
        for row in &self.rows {
            print_row(f, row)?;
        }

        // print bottom border
        writeln!(f, "+{:1$}+", "", total_width)
    }
}

//! # Description
//! Simple xlsx to csv converter
//!
//! # Usage
//! please get help by following command.
//! ```
//! xq --help
//! ```
//!
//! # convert to JSON
//! The script does not support for json output.
//! If you want to get json, you can use jq for csv to json conversion.
//! ```
//! xq file.xlsx Sheet1 | jq -R -s -f mapping.jq
//! ```
//!
//! where, `mapping.jq` is:
//!
//!```
//! split("\n")
//!   |map(split(","))
//!   |map({
//!     "A":.[0],
//!     "B":.[1],
//!     "C":.[2],
//!   })
//! ```

use calamine::{open_workbook, DataType, Reader, Xlsx};
use clap::Parser;
use csv::Writer;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// file path to process [.xlsx]
    file: String,

    /// sheet name to process
    sheet: String,
}

fn main() {
    let cli = Cli::parse();

    let mut wb: Xlsx<_> = open_workbook(&cli.file).expect("cannot read file");
    let range = wb.worksheet_range(&cli.sheet).expect("cannot read sheet");

    let stdout = std::io::stdout().lock();
    let mut wtr = Writer::from_writer(stdout);

    for row in range.rows() {
        let v = row
            .iter()
            .map(|d| d.as_string().unwrap_or("".to_owned()))
            .collect::<Vec<_>>();
        let _ = wtr.write_record(v);
    }
}

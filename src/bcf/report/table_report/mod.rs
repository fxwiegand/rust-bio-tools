mod alignment_reader;
mod create_report_table;
mod fasta_reader;
mod static_reader;

use crate::bcf::report::table_report::create_report_table::make_table_report;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

pub fn table_report(
    vcf: &str,
    fasta: &str,
    bam: &str,
    output_path: &str,
    sample: &str,
) -> Result<(), Box<dyn Error>> {
    let reports = make_table_report(Path::new(vcf), Path::new(fasta), Path::new(bam))?;

    let detail_path = output_path.to_owned() + "/details/" + sample;
    fs::create_dir(Path::new(&detail_path))?;

    for (gene, report_data) in reports {
        let mut templates = Tera::default();
        templates
            .add_raw_template(
                "table_report.html.tera",
                include_str!("report_table.html.tera"),
            )
            .unwrap();
        let mut context = Context::new();
        context.insert("variants", &report_data);
        context.insert("gene", &gene);

        let html = templates
            .render("table_report.html.tera", &context)
            .unwrap();
        let filepath = String::from(detail_path.clone()) + "/" + &gene + ".html";
        let mut file = File::create(filepath)?;
        file.write_all(html.as_bytes())?;
    }

    Ok(())
}
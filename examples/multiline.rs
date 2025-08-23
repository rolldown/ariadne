use rolldown_ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};

fn main() {
    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);
    let out2 = colors.next();

    Report::build(ReportKind::Error, ("sample.tao", 32..33))
        .with_code(3)
        .with_message("Incompatible types".to_string())
        .with_help(format!(
            "Outputs of {} expressions must coerce to the same type",
            "match".fg(out)
        ))
        .finish()
        .print(("sample.tao", Source::from(include_str!("sample.tao"))))
        .unwrap();
}

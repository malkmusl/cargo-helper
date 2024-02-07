use std::io::Write;

use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };

use crate::project::license::get_time;

pub fn print_error(err: &str) -> String {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    let time = get_time();
    write!(&mut stdout, "[{}]  [", time).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();

    write!(&mut stdout, "ERROR").unwrap();

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    write!(&mut stdout, "]  -> ").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
    write!(&mut stdout, "{}\n", err).unwrap();
    stdout.reset().unwrap();
    err.to_owned()
}

pub fn print_warn(err: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    let time = get_time();
    write!(&mut stdout, "[{}]  [", time).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();

    write!(&mut stdout, "WARN").unwrap();

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    write!(&mut stdout, "]   -> ").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();
    write!(&mut stdout, "{}\n", err).unwrap();
    stdout.reset().unwrap();
}

pub fn print_success(err: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    let time = get_time();
    write!(&mut stdout, "[{}] [", time).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap();

    write!(&mut stdout, "SUCCESS").unwrap();

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    write!(&mut stdout, "] -> ").unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap();
    write!(&mut stdout, "{}\n", err).unwrap();
    stdout.reset().unwrap();
}

pub fn print_info(err: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    let time = get_time();
    write!(&mut stdout, "[{}]  [", time).unwrap();

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(0, 128, 255)))
                .set_bold(true)
        )
        .unwrap();

    write!(&mut stdout, "INFO").unwrap();

    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Rgb(64, 64, 64)))
                .set_bold(true)
        )
        .unwrap();

    write!(&mut stdout, "]   -> ").unwrap();

    stdout.reset().unwrap();
    write!(&mut stdout, "{}\n", err).unwrap();
}

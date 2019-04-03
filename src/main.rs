use ansi_term::Color::Yellow;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct App {
    /// The emoji to represent filled-in space
    #[structopt(short, default_value = "rust")]
    foreground: String,
    /// The emoji to represent negative space
    #[structopt(short)]
    background: Option<String>,
    #[structopt(short, default_value = "10")]
    max_chars: usize,
    /// The text to print in emoji
    text: String,
}

fn main() {
    let app = App::from_args();
    let moji = slack_moji::Moji::new(app.foreground, app.background, app.max_chars);
    let txt = moji.draw(&app.text);

    if txt.len() > 4000 {
        eprint!("{}: ", Yellow.bold().paint("WARNING"));
        eprint!("Output is greater than 4000 characters. ");
        eprintln!("Slack will not let you send it.");
    }

    println!("{}", txt);
}

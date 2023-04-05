use rusticlipboard::domain::clip::field::{Content, Expires, Password, ShortCode, Title};
use rusticlipboard::service::ask::{GetClip, NewClip, UpdateClip};
use rusticlipboard::web::api::{ApiKey, API_KEY_HEADER};
use rusticlipboard::Clip;
use std::error::Error;
use structopt::StructOpt;
use strum::EnumString;

#[derive(Debug, StructOpt)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expires")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
    Update {
        #[structopt(help = "content")]
        clip: String,
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expires")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "clipclient", about = "Rusticlipboard API client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8080", env = "RUSTICLIPBOARD_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            shortcode,
            password,
        } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            todo!()
        }
        Command::New {
            clip,
            title,
            expires,
            password,
        } => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            todo!()
        }
        Command::Update {
            clip,
            shortcode,
            title,
            expires,
            password,
        } => todo!(),
    }
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e)
    }
}

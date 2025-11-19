use clap::{Arg, ArgMatches, Command};
use mdbook_inpage_tab::InPageTab;
use mdbook_preprocessor::MDBOOK_VERSION;
use mdbook_preprocessor::{Preprocessor, errors::Result};
use semver::{Version, VersionReq};
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("mdbook-inpage-tab-preprocessor")
        .about("A mdbook preprocessor which adds tab support to your book")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = InPageTab::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");

    if let Ok(true) = pre.supports_renderer(renderer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

use clap::Parser;

#[derive(Parser)]
#[command(author="Wessel Badenhorst", version, about="Forge new SaaS apps quickly")]
pub struct Args {
    pub project_name: Option<String>,

    #[arg(long)]
    pub no_install: bool,
}

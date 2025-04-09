use anyhow::Result;
#[cfg(feature = "gui")]
use fltk::dialog::message;
use reqwest::blocking::get;

fn main() -> Result<()> {
    let sites = [
        //"https://d2mutuy95x2dyc.cloudfront.net",
        "https://evalue.internationaldelivers.com/",
        "https://guiderestserver.azurewebsites.net/",
        "https://logs-navistar.s3.us-east-2.amazonaws.com",
        "https://msi.navistar.com/",
        "https://occwebsso.navistar.com/",
        "https://ws.oncommandconnection.com/",
        "https://xg.internationaldelivers.com/",
    ];

    for site in sites {
        eprint!("Updating {site}");
        let res = get(site).map(|_| "OK").unwrap_or("");
        eprintln!("{res:?}");
    }
    #[cfg(feature = "gui")]
    message(200, 200, "Done updating Root Certificates");
    Ok(())
}

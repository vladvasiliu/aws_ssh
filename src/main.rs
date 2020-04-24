use std::process::exit;

use fern;
use log::{error, info, LevelFilter};
use rusoto_core::Region;
use rusoto_ec2::Ec2Client;

use crate::aws::ec2::EC2Instance;

mod aws;
mod ip;

#[tokio::main]
async fn main() {
    setup_logger(LevelFilter::Debug).unwrap();
    let external_ip = ip::guess().await.unwrap_or_else(|_| exit(1));

    let ec2_client = Ec2Client::new(Region::EuWest3);
    match EC2Instance::from_query(String::from("i-1234"), ec2_client).await {
        Ok(_) => info!("Ok!"),
        Err(err) => error!("{}", err),
    }
}

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                // "[ {} ][ {:5} ][ {:15} ] {}",
                // chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                "[ {:5} ][ {:15} ] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        //        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

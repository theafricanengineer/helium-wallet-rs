use crate::{
    cmd::{api_url, load_wallet, Opts, OutputFormat},
    result::Result,
    wallet::Wallet,
};
use helium_api::{Account, Client, Hnt};
use prettytable::Table;
use qr2term::print_qr;
use serde_json::json;
use structopt::StructOpt;

/// Get wallet information
#[derive(Debug, StructOpt)]
pub struct Cmd {
    /// Display QR code for a given single wallet.
    #[structopt(long = "qr")]
    qr_code: bool,
}

impl Cmd {
    pub fn run(&self, opts: Opts) -> Result {
        let wallet = load_wallet(opts.files)?;
        if self.qr_code {
            let address = wallet.address()?;
            print_qr(&address)?;
            Ok(())
        } else {
            let client = Client::new_with_base_url(api_url());
            let account = client.get_account(&wallet.address()?)?;
            print_wallet(&wallet, &account, opts.format)
        }
    }
}

fn print_wallet(wallet: &Wallet, account: &Account, format: OutputFormat) -> Result {
    match format {
        OutputFormat::Table => {
            let mut table = Table::new();
            table.add_row(row!["Key", "Value"]);
            table.add_row(row!["Address", account.address]);
            table.add_row(row!["Sharded", wallet.is_sharded()]);
            table.add_row(row!["PWHash", wallet.pwhash()]);
            table.add_row(row!["Balance", Hnt::from_bones(account.balance)]);
            table.add_row(row!["DC Balance", account.dc_balance]);
            table.add_row(row!["Securities Balance", account.sec_balance]);
            table.printstd();
        }
        OutputFormat::Json => {
            let table = json!({
                "sharded": wallet.is_sharded(),
                "pwhash": wallet.pwhash().to_string(),
                "account": account,
            });
            println!("{}", serde_json::to_string_pretty(&table)?);
        }
    };
    Ok(())
}

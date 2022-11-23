#[cfg(unix)]
use std::collections::HashMap;
use std::env;

use heim::{
    process::{ProcessResult},
    process
};

pub(crate) fn process() -> ProcessResult<()> {
    smol::block_on(async {
        let process = match env::args().nth(2) {
            Some(value) => {
                let pid = value.parse().expect("cannot get process from this PID");
                process::get(pid).await?
            }
            None => {
                eprintln!(
                    "Process PID is not passed as an arg, self PID will be used instead"
                );
                process::current().await?
            }
        };
        let table = prettify(process).await?;
        table.print_tty(false);

        Ok(())
    })
}

async fn prettify(p: process::Process) -> process::ProcessResult<prettytable::Table> {
    let mut table = prettytable::Table::new();

    table.add_row(row!["PID", p.pid()]);
    table.add_row(row!["Parent PID", p.parent_pid().await?]);
    table.add_row(row!["Name", p.name().await?]);
    table.add_row(row!["Exe", p.exe().await?.display()]);

    #[cfg(unix)]
    {
        table.add_row(row!["Command", format!("{:?}", p.command().await?)]);
        table.add_row(row!["Current working dir", format!("{:?}", p.cwd().await?)]);
        table.add_row(row![
            "Environment",
            format!(
                "{:?}, ..",
                p.environment()
                    .await?
                    .iter()
                    .take(3)
                    .collect::<HashMap::<_, _>>()
            )
        ]);
    }

    table.add_row(row!["Status", format!("{:?}", p.status().await?)]);
    table.add_row(row!["Create time", format!("{:?}", p.create_time().await?)]);
    table.add_row(row!["CPU time", format!("{:?}", p.cpu_time().await?)]);

    Ok(table)
}
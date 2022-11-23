use heim::{
    memory,
    units::information,
    process::{ProcessResult}
};

pub(crate) fn free() -> ProcessResult<()> {
    show_mem_stats()
}

#[cfg(unix)]
fn show_mem_stats() -> ProcessResult<()> {
    smol::block_on(async {
        let memory = memory::memory().await?;
        let swap = memory::swap().await?;

        let total = memory.total().get::<information::megabyte>();
        let allocated = memory.active().get::<information::megabyte>();
        let available = memory.available().get::<information::megabyte>();

        let used = total - (allocated + available);

        println!("\t\tTotal\t\tUsed\t    Allocated\t    Free\t  Available");
        println!(
            "{:>7} {:>11?}_MB {:>11?}_MB {:>11?}_MB {:>11?}_MB {:>11?}_MB",
            "Mem:",
            total,
            used,
            allocated,
            memory.free().get::<information::megabyte>(),
            available,
        );
        println!(
            "{:>7} {:>11?}_MB {:>11?}_MB {:>11?}    {:>11?}_MB",
            "Swap:",
            swap.total().get::<information::megabyte>(),
            swap.used().get::<information::megabyte>(),
            0,
            swap.free().get::<information::megabyte>(),
        );

        Ok(())
    })
}

#[cfg(target_os = "windows")]
fn show_mem_stats() -> ProcessResult<()> {
    smol::block_on(async {
        let memory = memory::memory().await?;
        let swap = memory::swap().await?;

        let total = memory.total().get::<information::megabyte>();
        let available = memory.available().get::<information::megabyte>();

        let used = total - available;

        println!("\t\tTotal\t\tUsed\t      Free\t  Available");
        println!(
            "{:>7} {:>11?}_MB {:>11?}_MB {:>11?}_MB {:>11?}_MB",
            "Mem:",
            total,
            used,
            memory.free().get::<information::megabyte>(),
            available,
        );
        println!(
            "{:>7} {:>11?}_MB {:>11?}_MB {:>11?}    {:>11?}_MB",
            "Swap:",
            swap.total().get::<information::megabyte>(),
            swap.used().get::<information::megabyte>(),
            0,
            swap.free().get::<information::megabyte>(),
        );

        Ok(())
    })
}
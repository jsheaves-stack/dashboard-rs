use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

pub trait ListProcesses {
    // fn list() -> Result<Vec<u8>, String>;
    fn list();
}

pub struct ProcessExplorer;

impl ListProcesses for ProcessExplorer {
    fn list() {
        let mut sys = System::new_all();

        sys.refresh_all();

        // We display all disks' information:
        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        // Network interfaces name, data received and data transmitted:
        println!("=> networks:");
        for (interface_name, data) in sys.networks() {
            println!(
                "{}: {}/{} B",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }

        // Components temperature:
        println!("=> components:");
        for component in sys.components() {
            println!("{:?}", component);
        }

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} KB", sys.total_memory());
        println!("used memory : {} KB", sys.used_memory());
        println!("total swap  : {} KB", sys.total_swap());
        println!("used swap   : {} KB", sys.used_swap());

        // Display system information:
        println!("System name:             {:?}", sys.name());
        println!("System kernel version:   {:?}", sys.kernel_version());
        println!("System OS version:       {:?}", sys.os_version());
        println!("System host name:        {:?}", sys.host_name());

        // Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

        // Display processes ID, name na disk usage:
        for (pid, process) in sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }
    }
}

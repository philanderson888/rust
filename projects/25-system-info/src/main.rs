use sysinfo::{
    Components, Disks, Networks, System,
};


fn main() {

    println!("==============================================================");
    println!("==============================================================");
    println!("====               System Information                     ====");
    println!("==============================================================");
    println!("==============================================================");

    
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("==============================================================");
    println!("====                     Memory                           ====");
    println!("==============================================================");
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    println!("==============================================================");
    println!("====                Operating System                      ====");
    println!("==============================================================");
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    println!("==============================================================");
    println!("====                       CPUS                           ====");
    println!("==============================================================");
    println!("NB CPUs: {}", sys.cpus().len());

    println!("==============================================================");
    println!("====                 System Processes                     ====");
    println!("==============================================================");
    let mut counter = 0;
    for (pid, process) in sys.processes() {
        counter += 1;
        if counter >= 10 {
            break;
        }
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }

    println!("==============================================================");
    println!("====                       Disks                          ====");
    println!("==============================================================");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}\n");
    }

    println!("==============================================================");
    println!("====                     Networks                         ====");
    println!("==============================================================");
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} bytes received / {} bytes sent",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }

    
    println!("==============================================================");
    println!("====                    Temperature                       ====");
    println!("==============================================================");
    let components = Components::new_with_refreshed_list();
    let mut counter = 0;
    for component in &components {
        counter += 1;
        if counter >= 10 {
            break;
        }
        println!("{component:?}");
    }

}

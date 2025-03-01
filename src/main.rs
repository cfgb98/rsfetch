use sysinfo::{Disks, System};

fn main() {
  // Obtener información del sistema
  let mut sys = System::new_all();
  sys.refresh_all();
// Display system information:
println!("Bienvenido a  RSfetch");
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());

  //RAM INFO
 
  println!("total memory: {} bytes", sys.total_memory());
  println!("used memory : {} bytes", sys.used_memory());
  println!("total swap  : {} bytes", sys.total_swap());
  println!("used swap   : {} bytes", sys.used_swap());

// Number of CPUs:
println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name na disk usage:
for (pid, process) in sys.processes() {
    println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
}

// We display all disks' information:
println!("=> disks:");
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{disk:?}");
}
   
}

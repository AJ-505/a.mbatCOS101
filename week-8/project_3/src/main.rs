use std::fs::File;
use std::io::Write;
fn main() {
    let mut file = File::create("convicted_ministers.csv").expect("File creation failed");
    let com_names = ["Aigbogun Alamba Daudu", "Muritala Afeez Bendu", "Okorocha Calistus Ogbonna", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zones = ["South West", "North East", "South South", "South West", "South East"];

    file.write_all("S/N,Name of Commisioner, Ministry, Geopolitical Zone\n".as_bytes()).expect("Failed to write to file");
    for i in 0..5 {
        file.write_all(format!("{},", i+1).as_bytes()).expect("Failed to write to file");
        file.write_all(format!("{},", com_names[i]).as_bytes()).expect("Failed to write to file");
        file.write_all(format!("{},", ministries[i]).as_bytes()).expect("Failed to write to file");
        file.write_all(format!("{}\n", geo_zones[i]).as_bytes()).expect("Failed to write to file");
    }
    println!("The results have been successfully saved into the file convicted_ministers.csv.\nThank you for using my program!");
}

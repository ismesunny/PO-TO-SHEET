use std::path::Path;
use subprocess::Exec;

fn main() {
    let input_po = String::from("data/import_po/name_file.po"); //import po file
    let output_csv = String::from("data/export_csv/name_filecsv"); //export csv file
    let output_po = String::from("data/export_po/name_file.po"); //export to po file
    check_dep();
    po2csv(input_po.clone(), output_csv.clone());
    csv2po(input_po, output_csv, output_po);
}

fn check_dep() {
    let file_npm = Path::new("/usr/bin/npm");

    if file_npm.exists() {
        println!("npm exist")
    } else {
        Exec::shell("sudo pacman -S npm --nocofirm").join().unwrap();
    }

    let file_node = Path::new("/usr/bin/node");

    if file_node.exists() {
        println!("nodejs exist")
    } else {
        Exec::shell("sudo pacman -S nodejs --nocofirm")
            .join()
            .unwrap();
    }
}
fn po2csv(p2c_input_po: String, p2c_output_csv: String) {
    let convert = format!("node index.js {} > {}", p2c_input_po, p2c_output_csv);
    println!("test {}", convert);
    //-----------------(use index.js)---------------(Folder's po)------------(where to convert into)
    Exec::shell(convert).join().unwrap();
}
fn csv2po(c2p_input_po: String, c2p_output_csv: String, c2p_output_po: String) {
    let convert = format!(
        "node index.js {} {} > {}",
        c2p_input_po, c2p_output_csv, c2p_output_po
    );
    println!("test {}", convert);
    //-----------------(use index.js)---------------(Folder's po)------------(where to convert into)
    Exec::shell(convert).join().unwrap();
}

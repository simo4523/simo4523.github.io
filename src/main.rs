use std::io::Read;

fn main() {
    struct Page {
        folder_name: &'static str,
        target_file_name: &'static str,
    }

    const PAGES: [Page; 3] = [
        Page {
            folder_name: "crypto",
            target_file_name: "crypto",
        },
        Page {
            folder_name: "fl-studio",
            target_file_name: "fl-studio",
        },
        Page {
            folder_name: "home",
            target_file_name: "index",
        },
    ];

    let base_filename = "base.html";
    let mut base_contents = String::new();
    if let Ok(mut f) = std::fs::File::open(&base_filename) {
        if let Err(e) = f.read_to_string(&mut base_contents) {
            eprintln!("Failed to read {}: {}", base_filename, e);
        }
    }

    for page in PAGES.iter() {
        let filename = format!("{}/index.html", page.folder_name);
        let file = std::fs::File::open(&filename);
        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    eprintln!("Failed to read {}: {}", filename, e);
                } else {
                    let new_filename = format!("{}.html", page.target_file_name);
                    if let Err(e) = std::fs::write(
                        &new_filename,
                        base_contents
                            .as_str()
                            .replace("%title%", page.folder_name)
                            .replace("%content%", &contents),
                    ) {
                        eprintln!("Failed to write {}: {}", new_filename, e);
                    } else {
                        println!("Created file: {}", new_filename);
                    }
                }
            }
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
        }
    }
}

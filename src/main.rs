use std::io::Read;

fn main() {
    struct Page {
        folder_name: &'static str,
        target_file_name: &'static str,
    }

    const PAGES: [Page; 4] = [
        Page {
            folder_name: "home",
            target_file_name: ".",
        },
        Page {
            folder_name: "games",
            target_file_name: "games",
        },
        Page {
            folder_name: "crypto",
            target_file_name: "crypto",
        },
        Page {
            folder_name: "fl-studio",
            target_file_name: "fl-studio",
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
        let filename = format!("pages/{}.html", page.folder_name);
        let file = std::fs::File::open(&filename);
        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    eprintln!("Failed to read {}: {}", filename, e);
                } else {
                    let new_filename = format!("docs/{}/index.html", page.target_file_name);
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

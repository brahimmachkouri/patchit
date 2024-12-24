/*
   (c) Brahim MACHKOURI 2024
*/
use std::collections::HashMap;
use std::env;
use std::fs;
//use std::io::{Read, Write, Seek, SeekFrom};
use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::process;

#[derive(Serialize, Deserialize)]
struct Patch {
    offset: u64,
    data: String,
}

#[derive(Serialize, Deserialize)]
struct PatchFile {
    file_name: String,
    checksum: String,
    patches: Vec<Patch>,
}

fn compute_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn create_patch_file(file_name: String, checksum: String, patches: Vec<Patch>) -> PatchFile {
    PatchFile {
        file_name,
        checksum,
        patches,
    }
}

fn replace_extension_with_json(filename: &str) -> String {
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
    format!("{}.json", stem)
}

fn generate_patch(original_file: &str, modified_file: &str, output_patch: &str) -> Result<(), String> {
    let original_data = fs::read(original_file).map_err(|e| format!("Unable to read the original file: {}", e))?;
    let modified_data = fs::read(modified_file).map_err(|e| format!("Unable to read the modified file: {}", e))?;

    if original_data.len() != modified_data.len() {
        return Err("The files do not have the same size, comparison not possible".to_string());
    }

    let checksum = compute_checksum(&original_data);

    let mut patches = Vec::new();
    for (i, (&orig_byte, &mod_byte)) in original_data.iter().zip(&modified_data).enumerate() {
        if orig_byte != mod_byte {
            patches.push(Patch {
                offset: i as u64,
                data: hex::encode([mod_byte]),
            });
        }
    }

    let patch_file = create_patch_file(modified_file.to_string(), checksum, patches);
    let patch_json = serde_json::to_string_pretty(&patch_file).map_err(|e| format!("Error generating the JSON: {}", e))?;

    fs::write(output_patch, patch_json).map_err(|e| format!("Unable to write the JSON file: {}", e))?;
    Ok(())
}

fn apply_patch_file(patch_file_path: &str) -> Result<(), String> {
    let patch_data = fs::read_to_string(patch_file_path).map_err(|e| format!("Unable to read the patch file: {}", e))?;
    let patch_file: PatchFile = serde_json::from_str(&patch_data).map_err(|e| format!("Invalid JSON format: {}", e))?;

    if !Path::new(&patch_file.file_name).exists() {
        return Err(format!("The target file '{}' does not exist", patch_file.file_name));
    }

    let target_data = fs::read(&patch_file.file_name).map_err(|e| format!("Unable to read the target file: {}", e))?;
    let actual_checksum = compute_checksum(&target_data);

    if actual_checksum != patch_file.checksum {
        return Err("Invalid checksum: the target file does not match the original file".to_string());
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(&patch_file.file_name)
        .map_err(|e| format!("Unable to open the target file: {}", e))?;

    for patch in patch_file.patches {
        let data = hex::decode(&patch.data).map_err(|e| format!("Invalid hexadecimal data at offset 0x{:X}: {}", patch.offset, e))?;
        file.seek(SeekFrom::Start(patch.offset)).map_err(|e| format!("Unable to seek to offset 0x{:X}: {}", patch.offset, e))?;
        file.write_all(&data).map_err(|e| format!("Failed to write at offset 0x{:X}: {}", patch.offset, e))?;
    }

    Ok(())
}

fn print_help() {
    println!("Usage: patchit [OPTIONS]");
    println!("\nOptions:");
    println!("  --source, -s <file>    Path of the source/original file (generate mode)");
    println!("  --modified, -m <file>  Path of the modified file (generate mode)");
    println!("  --output, -o <file>    Name of the output JSON file (default: modified.json)");
    println!("  --help, -h             Display this help message");
    println!("\nExamples:");
    println!("  ./patchit --source MyApplication.old --modified MyApplication [--output mypatch.json]");
    println!("  ./patchit -s MyApplication.old -m MyApplication [-o mypatch.json]");
    println!("  ./patchit mypatch.json");
    println!("\n  .\\patchit.exe -s gimp.orig.exe -m gimp.exe [-o gimp.json]");
    println!("  .\\patchit.exe mypatch.json");
}

fn main() {
    let args: HashMap<String, String> = env::args()
    .skip(1)
    .filter(|arg| arg.starts_with("--") || arg.starts_with("-"))
    .flat_map(|arg| {
        let mut split = arg.splitn(2, '=');
        Some((
            split.next()?.to_string(),
            split.next().unwrap_or("").to_string(),
        ))
    })
    .collect();

    if args.contains_key("--help") || args.contains_key("-h") {
        print_help();
        process::exit(0);
    }

    let source = args.get("--source").or_else(|| args.get("-s"));
    let modified = args.get("--modified").or_else(|| args.get("-m"));
    let output = args.get("--output").or_else(|| args.get("-o"));

    if let (Some(source), Some(modified)) = (source, modified) {
        let output = match output {
            Some(o) => o.to_string(),
            None => replace_extension_with_json(&modified),
        };
    
        if let Err(e) = generate_patch(source, modified, &output) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        println!("Patch file successfully generated: {}", output);
    } else if let Some(patch_file) = env::args().nth(1) {
        if let Err(e) = apply_patch_file(&patch_file) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        println!("Patches applied successfully.");
    } else {
        eprintln!("Error: Missing required arguments. Use --help for usage.");
        process::exit(1);
    }
}

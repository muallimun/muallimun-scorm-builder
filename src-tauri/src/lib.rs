use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use tauri_plugin_dialog::DialogExt;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn process_desktop_scorm(
    app: tauri::AppHandle,
    version: String,
    title: String,
) -> Result<serde_json::Value, String> {
    let folder_path_buf = app
        .dialog()
        .file()
        .blocking_pick_folder()
        .ok_or("İşlem iptal edildi veya klasör seçilmedi.")?;
    let folder_path = folder_path_buf
        .into_path()
        .map_err(|_| "Klasör yolu okunamadı.")?;

    let folder_name = folder_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let safe_title = if title.trim().is_empty() {
        folder_name.clone()
    } else {
        title.clone()
    };

    let mut files_xml = String::new();
    let mut launch_file = String::new();
    let launch_candidates = ["index_lms.html", "story.html", "index.html", "main.html"];

    for entry in WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name == "imsmanifest.xml" || file_name == ".DS_Store" {
                continue;
            }

            let rel_path = entry_path
                .strip_prefix(&folder_path)
                .unwrap()
                .to_string_lossy()
                .replace("\\", "/");
            files_xml.push_str(&format!("      <file href=\"{}\" />\n", rel_path));

            if launch_file.is_empty()
                && launch_candidates.contains(&file_name.as_str())
                && !rel_path.contains("/")
            {
                launch_file = file_name;
            }
        }
    }

    if launch_file.is_empty() {
        launch_file = "index.html".to_string();
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let manifest_id = format!("MANIFEST_{}", timestamp);
    let org_id = format!("ORG_{}", timestamp);
    let item_id = format!("ITEM_{}", timestamp);
    let res_id = format!("RES_{}", timestamp);

    let xml_content = if version == "1.2" {
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<manifest identifier=\"{}\" version=\"1.0\" xmlns=\"http://www.imsproject.org/xsd/imscp_rootv1p1p2\" xmlns:adlcp=\"http://www.adlnet.org/xsd/adlcp_rootv1p2\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.imsproject.org/xsd/imscp_rootv1p1p2 imscp_rootv1p1p2.xsd http://www.imsglobal.org/xsd/imsmd_rootv1p2p1 imsmd_rootv1p2p1.xsd http://www.adlnet.org/xsd/adlcp_rootv1p2 adlcp_rootv1p2.xsd\">\n  <metadata><schema>ADL SCORM</schema><schemaversion>1.2</schemaversion></metadata>\n  <organizations default=\"{}\"><organization identifier=\"{}\"><title>{}</title><item identifier=\"{}\" isvisible=\"true\" identifierref=\"{}\"><title>{}</title></item></organization></organizations>\n  <resources><resource identifier=\"{}\" type=\"webcontent\" href=\"{}\" adlcp:scormtype=\"sco\">\n{}\n    </resource></resources>\n</manifest>",
            manifest_id,
            org_id,
            org_id,
            safe_title,
            item_id,
            res_id,
            safe_title,
            res_id,
            launch_file,
            files_xml
        )
    } else {
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<manifest identifier=\"{}\" version=\"1.0\" xmlns=\"http://www.imsglobal.org/xsd/imscp_v1p1\" xmlns:adlcp=\"http://www.adlnet.org/xsd/adlcp_v1p3\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.imsglobal.org/xsd/imscp_v1p1 imscp_v1p1.xsd http://www.adlnet.org/xsd/adlcp_v1p3 adlcp_v1p3.xsd\">\n  <metadata><schema>ADL SCORM</schema><schemaversion>2004 4th Edition</schemaversion></metadata>\n  <organizations default=\"{}\"><organization identifier=\"{}\"><title>{}</title><item identifier=\"{}\" identifierref=\"{}\"><title>{}</title></item></organization></organizations>\n  <resources><resource identifier=\"{}\" type=\"webcontent\" href=\"{}\" adlcp:scormType=\"sco\">\n{}\n    </resource></resources>\n</manifest>",
            manifest_id,
            org_id,
            org_id,
            safe_title,
            item_id,
            res_id,
            safe_title,
            res_id,
            launch_file,
            files_xml
        )
    };

    // --- GÜNCELLEME 1: Eski dosyanın varlığını kontrol et ---
    let manifest_path = folder_path.join("imsmanifest.xml");
    let manifest_existed = manifest_path.exists();

    std::fs::write(&manifest_path, xml_content)
        .map_err(|e| format!("Manifest yazılamadı: {}", e))?;

    let parent_dir = folder_path.parent().unwrap_or(&folder_path);
    let zip_file_name = format!("{}_SCORM.zip", folder_name);
    let zip_file_path = parent_dir.join(&zip_file_name);

    let zip_file =
        File::create(&zip_file_path).map_err(|e| format!("Zip dosyası oluşturulamadı: {}", e))?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        let name = entry_path
            .strip_prefix(&folder_path)
            .unwrap()
            .to_string_lossy()
            .replace("\\", "/");
        if name.is_empty() {
            continue;
        }

        if entry_path.is_file() {
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            let mut f = File::open(entry_path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        } else if entry_path.is_dir() {
            zip.add_directory(name, options)
                .map_err(|e| e.to_string())?;
        }
    }
    zip.finish()
        .map_err(|e| format!("Zip tamamlanamadı: {}", e))?;

    // --- GÜNCELLEME 2: JavaScript'e detaylı JSON dön ---
    Ok(serde_json::json!({
        "path": parent_dir.to_string_lossy().to_string(),
        "zip_name": zip_file_name,
        "manifest_updated": manifest_existed
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_desktop_scorm, open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

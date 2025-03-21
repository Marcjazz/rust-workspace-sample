use axum::{extract::Multipart, response::IntoResponse};
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

/// Saves the uploaded file to the `uploads` directory
async fn save_file(file_name: String, data: &[u8]) -> io::Result<()> {
    let save_path = PathBuf::from("uploads").join(file_name);
    let mut file = File::create(save_path)?; // Create a new file in the uploads directory
    file.write_all(data) // Write the received data into the file
}

/// Handles file uploads from a multipart form request
pub async fn upload_files(mut multipart: Multipart) -> impl IntoResponse {
    while let Ok(Some(mut field)) = multipart.next_field().await {
        // Extract file name and content if available
        let file_name = String::from(field.file_name().unwrap_or("unnamed"));

        while let (file_name, Some(data)) = (
            file_name.to_owned(),
            match field.chunk().await {
                Err(_) => return format!("Failed to read chunk!"),
                Ok(chuck) => chuck,
            },
        ) {
            // Attempt to save the file
            if let Err(e) = save_file(file_name, &data).await {
                return format!("Failed to save file: {}", e);
            }
        }
    }

    String::from("File uploaded successfully")
}

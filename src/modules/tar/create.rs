use futures::FutureExt;
use tokio::io::{self};
use tokio::sync::Mutex;
use tracing::{event, Level};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tar::Builder;

pub async fn create(folder_path: &Path) -> Result<PathBuf, io::Error> {
  event!(Level::INFO, "Creating tarball from folder: {}", folder_path.display());

  // Create a tarball at the specified location
  let folder_path = folder_path.canonicalize()?;
  #[allow(clippy::unwrap_used)] // Since we used canonicalize, this shouldn't panic
  let mut tarball_name = folder_path.file_name().unwrap().to_os_string();
  tarball_name.push(".tar.gz");
  let tarball_path = folder_path.parent()
    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid folder"))?
    .join(tarball_name);
  let tarball_file = File::create(&tarball_path)?;

  // Create a gzip encoder
  // let gzip_encoder = flate2::write::GzEncoder::new(tarball_file, Compression::default());

  // Create a tar writer
  let tar_writer = Builder::new(BufWriter::new(tarball_file));
  let tar_writer = Arc::new(Mutex::new(tar_writer));

  // Iterate through the contents of the folder
  add_folder_contents(tar_writer.clone(), folder_path).await?;

  let _ = tar_writer.lock().await.finish();

  event!(Level::DEBUG, "Finished creating tarball");

  Ok(tarball_path)
}

fn add_folder_contents<W>(tar_writer: Arc<Mutex<Builder<W>>>, folder_path: PathBuf) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<(), io::Error>> + std::marker::Send>>
where
    W: std::marker::Unpin + std::io::Write + std::marker::Send + 'static,
{
    async move {
      let mut dir = tokio::fs::read_dir(folder_path).await?;
      while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let name = path.file_name().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name"))?;

        if path.is_file() {
          let file = tokio::fs::File::open(&path).await?;
          tar_writer.lock().await.append_file(name, &mut file.into_std().await)?;
        } else if path.is_dir() {
          tar_writer.lock().await.append_dir_all(name, &path)?;
        }
      }
      Ok(())
    }.boxed()
}
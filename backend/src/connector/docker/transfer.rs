use std::{io::Cursor, path::Path};

use bollard::{
  Docker, body_full,
  query_parameters::{DownloadFromContainerOptionsBuilder, UploadToContainerOptionsBuilder},
};
use centaurus::{bail, error::Result};
use eyre::Context;
use futures::TryStreamExt;
use tar::{Archive, Builder, Header};

pub trait DockerTransferExt {
  async fn download_file(&self, container: &str, path: &str) -> Result<Vec<u8>>;
  async fn upload_file(
    &self,
    container: &str,
    path: &str,
    file_name: &str,
    data: Vec<u8>,
  ) -> Result<()>;
}

impl DockerTransferExt for Docker {
  async fn download_file(&self, container: &str, path: &str) -> Result<Vec<u8>> {
    let options = Some(
      DownloadFromContainerOptionsBuilder::new()
        .path(path)
        .build(),
    );

    let stream = self.download_from_container(container, options);
    let bytes = stream
      .try_collect::<Vec<_>>()
      .await
      .context("Failed to download file")?
      .concat();

    let path = Path::new(path)
      .file_name()
      .and_then(|p| p.to_str())
      .map(|s| s.to_string())
      .ok_or_else(|| eyre::eyre!("Invalid file name"))?;

    data_from_tar(bytes, &path)
  }

  async fn upload_file(
    &self,
    container: &str,
    path: &str,
    file_name: &str,
    data: Vec<u8>,
  ) -> Result<()> {
    let options = Some(UploadToContainerOptionsBuilder::new().path(path).build());
    let body = body_full(tar_from_data(file_name, data)?.into());

    self
      .upload_to_container(container, options, body)
      .await
      .context("Failed to upload tar to container")?;

    Ok(())
  }
}

fn tar_from_data(path: &str, data: Vec<u8>) -> Result<Vec<u8>> {
  let mut tar_builder = Builder::new(Vec::new());

  let mut header = Header::new_gnu();
  header.set_size(data.len() as u64);
  header.set_cksum();

  tar_builder.append_data(&mut header, path, Cursor::new(data))?;
  Ok(tar_builder.into_inner()?)
}

fn data_from_tar(tar_data: Vec<u8>, path: &str) -> Result<Vec<u8>> {
  let mut archive = Archive::new(Cursor::new(tar_data));

  for entry in archive.entries()? {
    let entry = entry?;
    if entry.path()? == Path::new(path) {
      return Ok(entry.path_bytes().to_vec());
    }
  }

  bail!("file not found in tar: {}", path);
}

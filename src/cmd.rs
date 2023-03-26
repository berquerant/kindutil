use crate::exec::exec;
use crate::yml::Cluster;
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

pub fn delete_cluster(dir: &Path, name: &str) -> anyhow::Result<()> {
    exec("kind", vec!["delete", "cluster", "--name", name]).context("kind delete")?;
    let mut path = PathBuf::from(dir);
    path.push(format!("{}.yml", name));
    if path.exists() {
        fs::remove_file(path).context("remove kind yml")
    } else {
        Ok(())
    }
}

pub fn create_cluster(dir: &Path, name: &str, control_plane: u8, worker: u8) -> anyhow::Result<()> {
    let cluster = Cluster::generate(control_plane, worker);
    let yml = serde_yaml::to_string(&cluster).with_context(|| {
        format!(
            "serialize cluster: control_plane: {}, worker: {}",
            control_plane, worker
        )
    })?;

    let mut path = PathBuf::from(dir);
    path.push(format!("{}.yml", name));
    fs::create_dir_all(dir).context("create directory")?;
    fs::write(&path, yml).context("write kind yaml")?;

    exec(
        "kind",
        vec![
            "create",
            "cluster",
            "--name",
            name,
            "--config",
            path.to_str().unwrap(),
        ],
    )
    .context("create cluster")
}

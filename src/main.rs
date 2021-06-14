use anyhow::*;
use directories::UserDirs;
use env_logger::Builder;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

fn get_verion_file_path(only_directory: bool, with_under_score: bool) -> Result<PathBuf> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            if only_directory {
                return Ok(document_dir.join(r"SEGA\PHANTASYSTARONLINE2"));
            }

            if with_under_score {
                return Ok(document_dir.join(r"SEGA\PHANTASYSTARONLINE2\_version.ver"));
            } else {
                return Ok(document_dir.join(r"SEGA\PHANTASYSTARONLINE2\version.ver"));
            }
        } else {
            bail!("Not found User Documents directory.");
        }
    } else {
        bail!("Not found User directory.");
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => match event.kind {
                EventKind::Remove(_) => {
                    if &event.paths.len() != &0 {
                        let removed_path = &event.paths[0];

                        if removed_path.file_name().unwrap_or(OsStr::new("foo")) != "version.ver" {
                            continue;
                        }

                        if let Ok(under_score_version_file) = get_verion_file_path(false, true) {
                            if !under_score_version_file.exists() {
                                error!("Not found _version.ver file");
                            }

                            if let Ok(_) = fs::copy(&under_score_version_file, &removed_path) {
                                info!("Success to copy _version.ver to version.ver");
                            }
                        };
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut builder = Builder::new();
    builder.filter(None, log::LevelFilter::Trace);
    builder.init();

    info!("NGS fast loader started");

    let path = get_verion_file_path(true, false)?;

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e)
        }
    });

    Ok(())
}

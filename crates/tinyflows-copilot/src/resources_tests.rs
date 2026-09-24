// SPDX-License-Identifier: GPL-3.0-or-later
use super::*;

fn resource_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resources/flow-authoring")
}

#[test]
fn flow_authoring_manifest_matches_files_on_disk() {
    let root = resource_dir();
    let mut on_disk = Vec::new();
    let mut pending = vec![root.clone()];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(dir)
            .expect("read resource directory")
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else {
                on_disk.push(
                    path.strip_prefix(&root)
                        .expect("resource under root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    on_disk.sort();

    let mut listed: Vec<_> = FLOW_AUTHORING_FILES.iter().map(|file| file.path).collect();
    listed.sort();
    assert_eq!(listed, on_disk);
}

#[test]
fn flow_authoring_manifest_links_every_resource() {
    for file in FLOW_AUTHORING_FILES {
        if file.path == "WORKFLOW.md" {
            continue;
        }
        assert!(
            FLOW_AUTHORING_WORKFLOW.contains(file.path),
            "manifest does not link {}",
            file.path
        );
    }
}

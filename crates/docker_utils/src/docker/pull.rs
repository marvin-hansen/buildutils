use std::process::Command;
use crate::{DockerError, DockerUtil};

impl DockerUtil{

    pub(crate) fn pull(
        &self,
        container_id: &str,
        image: &str,
        platform: Option<&str>,
    ) -> Result<(), DockerError> {
        // Example docker pull --platform linux/amd64  asia-northeast1-docker.pkg.dev/future-309012/image-repo/api_proxy:b422ae3
        self.dbg_print(&format!(
            "[pull_container_image]: Pull container image for: {container_id}."
        ));

        // construct initial command
        let mut cmd = Command::new("docker");
        cmd.arg("pull");

        if platform.is_some() {
            let p = platform.expect("Failed to unwrap Docker platform string");
            cmd.arg("--platform").arg(p);
        }

        // Add the image
        cmd.arg(image);

        self.dbg_print(&format!("[pull_container_image]: Pull command: {cmd:?}"));

        // Run the command & return error in case of failure
        match cmd.output() {
            Ok(out) => {
                if out.status.success() {
                    self.dbg_print(&format!(
                        "[pull_container_image]: success. Image Pulled {image}"
                    ));
                } else {
                    eprintln!(
                        "Error pulling container image {}: {}",
                        container_id,
                        String::from_utf8_lossy(&out.stderr)
                    );
                    return Err(DockerError::from(format!(
                        "Error starting container {}: {}",
                        container_id,
                        String::from_utf8_lossy(&out.stderr)
                    )));
                }

                Ok(())
            }
            Err(e) => {
                eprintln!();
                eprintln!("Error pulling container image {container_id}: {e}");
                eprintln!();
                panic!("")
            }
        }
    }

}
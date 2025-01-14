use crate::ServiceUtilError;

pub(crate) fn verify_binary_exists(
    dbg: bool,
    root_path: &'static str,
    binaries: &Vec<&'static str>,
) -> Result<(), ServiceUtilError> {
    for b in binaries {
        let path = format!("{root_path}/{b}");
        if dbg {
            println!("[VerifyBinary]: Checking if binary exists: {path}");
        }
        if !std::path::Path::new(&path).exists() {
            return Err(ServiceUtilError::BinaryNotFound(b.to_string()));
        }
    }

    Ok(())
}

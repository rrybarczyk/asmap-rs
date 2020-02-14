use crate::common::*;

/// Create a new file
fn create_new_file(path: impl AsRef<Path>) -> Result<std::fs::File, Error> {
    let path = path.as_ref();
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|io_error| Error::IoError {
            io_error,
            path: path.to_path_buf(),
        })
}

/// Writes the asn bottleneck result to a file
pub(crate) fn write_bottleneck(mrt_hm: HashMap<Address, u32>) -> Result<(), Error> {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let now = epoch.as_secs();
    let out_path = format!("data/2020-28-160000-data.{}.out", now);

    let mut file = create_new_file(&out_path).unwrap();

    for (key, value) in mrt_hm {
        let text = format!("{}/{}|{:?}", key.ip, key.mask.unwrap(), value);
        writeln!(file, "{:?}", &text).unwrap();
    }

    Ok(())
}

pub(crate) fn write_mrt_data(mrt_hm: &HashMap<Address, HashSet<Vec<u32>>>) -> Result<(), Error> {
    let file_name = format!("mrt-data.out");
    let mut file = create_new_file(&file_name).unwrap();

    for (key, value) in mrt_hm {
        let text = format!("{:?}: {:?}", key, value);
        writeln!(file, "{:?}", &text).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn writes_result_to_file() -> Result<(), Error> {
        let mut mrt_hm: HashMap<Address, u32> = HashMap::new();
        mrt_hm.insert(Address::from_str("195.66.225.77/0")?, 62240);
        mrt_hm.insert(Address::from_str("5.57.81.186/24")?, 13335);
        write_bottleneck(mrt_hm)?;
        Ok(())
    }
}

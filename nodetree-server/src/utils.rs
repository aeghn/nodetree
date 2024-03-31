pub fn split_uuid_to_file_name(uuid: &str) -> (String, String, String) {
    let trimed = uuid.replace("-", "");
    let trimed = trimed.trim();

    (trimed[0..2].into(), trimed[2..4].into(), trimed[5..].into())
}

#[cfg(test)]
mod test {
    use crate::utils::split_uuid_to_file_name;

    #[test]
    fn test() {
        println!("{:?}", split_uuid_to_file_name("skdfjlkasd-jfask-ldfj"));
    }
}
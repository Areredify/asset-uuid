fn main() {}

#[cfg(test)]
mod tests {
    use asset_uuid::asset_uuid;
    use uuid::Uuid;

    #[test]
    fn macro_test() {
        let x = asset_uuid!("936DA01F9ABD4d9d80C702AF85C822A9");
        let s = "936DA01F9ABD4d9d80C702AF85C822A9";

        assert_eq!(*Uuid::parse_str(s).unwrap().as_bytes(), x);
    }
}

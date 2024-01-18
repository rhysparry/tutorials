#[cfg(test)]
mod tests {
    #[test]
    fn test_serialize_tuple() {
        let tuple = ("five", 9);
        let json_str = serde_json::to_string(&tuple).unwrap();
        assert_eq!(json_str, "[\"five\",9]");
    }
}

/// Ensures that 'name' contains the client type.
pub fn client_test_name(name: String, client_type: String) -> String {
    if name.is_empty() {
        return client_type;
    }
    if name.contains("CLIENT") {
        return name.replace("CLIENT", &client_type);
    }
    format!("{} ({})", name, client_type)
}

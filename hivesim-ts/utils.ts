// Ensures that 'name' contains the client type.
export function client_test_name(name: string, client_type: string): string {
    if (name.length === 0) {
        return client_type;
    }
    if (name.includes("CLIENT")) {
        return name.replace("CLIENT", client_type);
    }
    return `${name} (${client_type})`;
}
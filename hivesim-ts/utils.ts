import { RLP } from "@ethereumjs/rlp";

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

export function decodeENR(string: string) {
    const enr = Buffer.from(string.slice(4), 'base64')
    const decoded = RLP.decode(enr)
    const [signature, seq] = decoded;
    const kvs = new Map();
    const signed = [seq];
    for (let i = 2; i < decoded.length; i += 2) {
        const k = decoded[i];
        const v = decoded[i + 1];
        kvs.set(k.toString(), v);
        signed.push(k, v);
    }
    return {
        c: new TextDecoder().decode(kvs.get('c')),
        seq,
        signature,
    }
}

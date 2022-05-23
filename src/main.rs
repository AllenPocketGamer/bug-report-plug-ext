use ic_agent::identity::{BasicIdentity, Secp256k1Identity};

fn main() {
    let bi = BasicIdentity::from_pem_file("./identity.pem");

    match bi {
        Ok(_) => println!("parse ./identity.pem from plug wallet to basic identity"),
        Err(e) => eprintln!("failed to parse ./identity.pem from plug wallet to basic identity, {}", e),
    };

    let si = Secp256k1Identity::from_pem_file("./identity.pem");

    match si {
        Ok(_) => println!("parse ./identity.pem from plug wallet to secp256k1 identity"),
        Err(e) => eprintln!("failed to parse ./identity.pem from plug wallet to secp256k1 identity, {}", e),
    };
}

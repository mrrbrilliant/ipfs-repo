use ipfs_api::IpfsClient;
use std::env;

#[actix_rt::main]
async fn main() {
    env::set_var("LIBP2P_FORCE_PNET", "1");
    // 1. remove all bootstart list
    let client = IpfsClient::default();

    match client.bootstrap_rm_all().await {
        Ok(bootstrap) => {
            if !bootstrap.peers.is_empty() {
                println!("Remove default peers:");
                bootstrap.peers.iter().for_each(|peer| println!("  {}", &peer))
            }
        }
        Err(e) => {
            eprintln!("Error removing bootstrap list: {}", &e);
            return;
        }
    }
    

    client.config_set_string("Addresses.Gateway", "/ip4/0.0.0.0/tcp/8080").await.unwrap();
    client.config_set_string("Addresses.API", "/ip4/0.0.0.0/tcp/5001").await.unwrap();

    match client.config_get_json("Addresses").await {
        Ok(config) => {
            println!("{:#?}", &config)
        },
        Err(e) => {
            eprintln!("Error getting node configuration: {}", &e)
        }
    }

    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("print help function") 
    } else {
        match args[0].as_ref() {
            "add" | "--add" | "a" | "-a" => {
                println!("run add function")
            },
            "delete" | "--delete" | "d" | "-d" => {
                println!("run delete function")
            },
            _ => println!("print help function") 
        }
    }

}


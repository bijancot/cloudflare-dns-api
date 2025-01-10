use reqwest::{header::HeaderMap};
use serde_json::{Value, json};
use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the domain name that will be added to cloudflare
    #[arg(short, long)]
    domain: String,

    /// The IP that want to be added 
    #[arg(short, long)]
    ip: String,
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main(){
    dotenv().ok();
    let zone_id = env::var("ZONE_ID").unwrap().to_string();
    let email = env::var("EMAIL").unwrap().to_string();
    let auth_key =env::var("AUTH_KEY").unwrap().to_string();
    let record_id =env::var("RECORD_ID").unwrap().to_string();

    // let args = Args::parse();
    // let domain = args.domain;
    // let ip = args.ip;
    // let _ = create_record(&domain,&ip).await.expect("oops 2");
    get_domain_detail(zone_id, record_id, email, auth_key).await.expect("set the zone id inside .env to make this work");
}
// #[tokio::main]
async fn get_domain_detail(zone_id_env: String, record_id_env: String, email_env: String, auth_key_env: String) -> Result<(), Box<dyn std::error::Error>>{
    let mut kepala = HeaderMap::new();
    kepala.insert("X-Auth-Email",email_env.parse().unwrap());
    kepala.insert("X-Auth-Key",auth_key_env.parse().unwrap());

    let ehe = reqwest::Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id_env, record_id_env);
    
    // Test env output here 
    println!("{}", url);
    
    let resp = ehe.get(url)
        .headers(kepala)
        .send()
        .await?
        .text()
        .await?;


    let json: serde_json::Value =serde_json::from_str(&resp)?;
    // test get value of json using specific field
    // let _zone_id = json.get("result").and_then(|value| value.get("id"));

    print!("{}",json);
    Ok(())
}

async fn _create_record(domain: &str, ip: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut kepala = HeaderMap::new();
    kepala.insert("X-Auth-Email","your-email".parse().unwrap());
    kepala.insert("X-Auth-Key","your-auth-key".parse().unwrap());
    kepala.insert("Content-Type","application/json".parse().unwrap());

    let ip_to_pass=ip;
    let domain_to_pass = domain;

    let json_format = r#"{"comment": "Domain verification record",
      "content": "&domain_to_pass",
      "name": "domain_name",
      "proxied": true,
      "ttl": 3600,
      "type": "A"}"#;


    let mut host_data: Value = serde_json::from_str(json_format).unwrap();
    let content = host_data.get_mut("content").expect("param is missing");
    *content = json!(ip_to_pass);
    let json_domain = host_data.to_string();

    let mut domain_data: Value = serde_json::from_str(&json_domain).unwrap();
    let name = domain_data.get_mut("name").expect("param is missing");
    *name = json!(domain_to_pass);
    let json_body = domain_data.to_string();


    // println!("{}",json_body);
    let ehe = reqwest::Client::new();
    let resp = ehe.post("https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dns_records")
        .headers(kepala)
        .body(json_body)
        .send()
        .await?
        .text()
        .await?;

    print!("{}",resp);
    Ok(())
}
use reqwest::{header::HeaderMap};
use serde_json::{Value, json};
use clap::Parser;

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

    let args = Args::parse();

    let domain = args.domain;
    let ip = args.ip;
    let _ = create_record(&domain,&ip).await.expect("oops 2");
}
// #[tokio::main]
async fn get_domain_detail() -> Result<(), Box<dyn std::error::Error>>{
    let mut kepala = HeaderMap::new();
    kepala.insert("X-Auth-Email","your-email".parse().unwrap());
    kepala.insert("X-Auth-Key","your-auth-key".parse().unwrap());


    let ehe = reqwest::Client::new();
    let resp = ehe.get("https://api.cloudflare.com/client/v4/zones/$ZONE_ID/dns_records/$RECORD_ID")
        .headers(kepala)
        .send()
        .await?
        .text()
        .await?;


    let json: serde_json::Value =serde_json::from_str(&resp)?;
    let _zone_id = json.get("result").and_then(|value| value.get("id"));

    print!("{}",json);
    Ok(())
}

async fn create_record(domain: &str, ip: &str) -> Result<(), Box<dyn std::error::Error>>{
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
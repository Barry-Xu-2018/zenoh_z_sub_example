use zenoh::Config;

#[tokio::main]
async fn main() {
    // Usage: official_z_sub [endpoint]   (default: tcp/127.0.0.1:7447, client mode)
    let endpoint = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "tcp/127.0.0.1:7447".to_string());

    let mut config = Config::default();
    config.insert_json5("mode", r#""client""#).unwrap();
    config
        .insert_json5("connect/endpoints", &format!(r#"["{endpoint}"]"#))
        .unwrap();
    // Don't discover peers via multicast; talk only to the given endpoint.
    config
        .insert_json5("scouting/multicast/enabled", "false")
        .unwrap();

    println!("Opening session to {endpoint}...");
    let session = zenoh::open(config).await.unwrap();

    println!("Declaring Subscriber on 'demo/example/**'...");
    let subscriber = session.declare_subscriber("demo/example/**").await.unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());
        println!(
            "[Subscriber] Received {} ('{}': '{}')",
            sample.kind(),
            sample.key_expr().as_str(),
            payload
        );
    }
}

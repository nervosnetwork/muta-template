use muta::Muta;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() {
    let muta = Muta::new("config/config.toml", "config/genesis.toml").unwrap();

    muta.run().await.unwrap()
}

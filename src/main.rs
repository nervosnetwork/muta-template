use muta::Muta;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() {
    let muta = Muta::new("devtools/chain/config.toml", "devtools/chain/genesis.toml").unwrap();

    muta.run().await.unwrap()
}

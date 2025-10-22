use std::net::TcpListener;

use geoman::run;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await?;
    Ok(())
}

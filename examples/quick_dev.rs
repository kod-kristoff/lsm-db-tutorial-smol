use lsm_db_tutorial_smol::Db;

fn main() {
    smol::block_on(async {
        if let Err(err) = real_main().await {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    })
}

async fn real_main() -> eyre::Result<()> {
    let mut db = Db::new("db").await?;

    db.put("foo".as_bytes(), "bar".as_bytes()).await?;
    db.put("baz".as_bytes(), "qux".as_bytes()).await?;
    db.put("foo".as_bytes(), "goo".as_bytes()).await?;
    println!(
        "{:?}",
        String::from_utf8(db.get("foo".as_bytes()).await?.unwrap())
    );
    Ok(())
}

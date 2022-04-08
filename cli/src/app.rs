use anyhow::{Context, Result};

use cw_core::{
    extensions::Extension,
    sqlx::{
        postgres::{PgConnectOptions, PgSslMode},
        query, Error, Executor, PgPool,
    },
    tables::Table,
    types::DbType,
};

// TODO: Add term colors

use crate::opt::{Command, Database, DatabaseOpt, DatabaseUri, Opt};

pub async fn app(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Database(DatabaseOpt { db, command }) => {
            let pool = open_pool(db).await?;
            match command {
                Database::Create => {
                    println!("\n- Loading extensions");
                    Extension::create_all(&pool, |(e, p)| {
                        if p {
                            println!("> Extension '{e}' has been loaded")
                        }
                    })
                    .await?;
                    println!("- Loading extensions: Done!");
                    println!("\n- Creating types");
                    DbType::create_all(&pool, |(t, r)| {
                        match &r {
                            Ok(_) => println!("> Type '{t}' has been created"),
                            Err(_) => (),
                        };
                        r
                    })
                    .await?;
                    println!("- Creating types: Done!");
                    println!("\n- Creating tables");
                    Table::create_all(&pool, |(t, r)| {
                        match &r {
                            Ok(_) => println!("> Table '{t}' has been created"),
                            Err(_) => (),
                        };
                        r
                    })
                    .await?;
                    println!("- Creating tables: Done!");
                }
                Database::Drop => {
                    println!("\n- Dropping tables");
                    Table::drop_all(&pool, |(t, r)| {
                        match &r {
                            Ok(_) => println!("> Table '{t}' has been dropped"),
                            Err(_) => (),
                        }
                        r
                    })
                    .await?;
                    println!("- Dropping tables: Done!");
                    println!("\n- Dropping types");
                    DbType::drop_all(&pool, |(t, r)| {
                        match &r {
                            Ok(_) => println!("> Type '{t}' has been dropped"),
                            Err(_) => (),
                        }
                        r
                    })
                    .await?;
                    println!("- Dropping types: Done!");
                    println!("\n- Unloading extensions");
                    Extension::drop_all(&pool, |(e, p)| {
                        if p {
                            println!("> Extension '{e}' has been unloaded")
                        }
                    })
                    .await?;
                    println!("- Unloading extensions: Done!");
                }
                Database::Check { fix } => {
                    // TODO: Optimize
                    println!("\n- Checking extensions");
                    for e in Extension::ALL {
                        print!("> Checking '{e}' extension: ");
                        if e.exists(&pool)
                            .await
                            .context("While checking extension existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Loaded");

                            if fix {
                                print!(">> Trying to load extension '{e}': ");
                                match pool.execute(query(e.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(_) => {
                                        println!("Failed\n");
                                    }
                                };
                            }
                        }
                    }

                    println!("\n- Checking types");
                    for t in DbType::ALL {
                        print!("> Checking '{t}' type: ");
                        if t.exists(&pool)
                            .await
                            .context("While checking type existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create table '{t}': ");
                                match pool.execute(query(t.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(_) => {
                                        println!("Failed\n");
                                    }
                                };
                            }
                        }
                    }

                    println!("\n- Checking tables");
                    for t in Table::ALL {
                        print!("> Checking '{t}' table: ");
                        if t.exists(&pool)
                            .await
                            .context("While creating table existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create table '{t}': ");
                                match pool.execute(query(t.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(_) => {
                                        println!("Failed\n");
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }
        Command::Generate { .. } => unimplemented!(),
    }

    Ok(())
}

/// Creates connection pool to database
async fn open_pool(uri: DatabaseUri) -> Result<PgPool, Error> {
    let options: PgConnectOptions = uri.inner.as_str().parse()?;

    // Connecting to database
    let pool = PgPool::connect_with(
        options
            .application_name("CW-CLI")
            .ssl_mode(PgSslMode::Prefer),
    )
    .await?;

    Ok(pool)
}

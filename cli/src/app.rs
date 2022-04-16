use std::sync::Arc;

use anyhow::{Context, Result};

use cw_core::{
    extensions::Extension,
    generator::Config,
    sqlx::{
        pool::PoolOptions,
        postgres::{PgConnectOptions, PgQueryResult, PgSslMode},
        query, Error, Executor, PgPool,
    },
    tables::Table,
    types::DbType,
};
use futures::future::{try_join_all, BoxFuture};

// TODO: Add term colors

use crate::opt::{Command, Database, DatabaseOpt, DatabaseUri, Opt, PoolOpts};

pub async fn app(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Database(DatabaseOpt { db, command }) => {
            let pool = open_pool(db, &opt.pool_opts).await?;
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
                                print!(">> Trying to create type '{t}': ");
                                match pool.execute(query(t.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
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
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                }
                Database::Truncate => {
                    println!("\n!!! Truncating database !!!\n");

                    Table::truncate(&pool, |(t, s)| {
                        if s {
                            println!("> Table '{t}' has been truncated!");
                        }
                    })
                    .await?;

                    println!("\n!!! Truncation done !!!\n");
                }
            }
        }
        Command::Generate { uri } => {
            let pool = Arc::new(open_pool(uri, &opt.pool_opts).await?);

            let gen = Config::default();

            print!("Generating sample data...");
            let (
                mut component_kind,
                mut service,
                mut position,
                mut manufacturer,
                mut person,
                mut supplier,
                mut labor_contract,
                mut staff,
                mut account,
            ) = gen.gen_full();
            println!(" : Done");

            // Low-level tables
            {
                println!("Inserting low-level tables data");
                let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> = Vec::with_capacity(
                    component_kind.len()
                        + service.len()
                        + position.len()
                        + manufacturer.len()
                        + person.len()
                        + supplier.len(),
                );

                component_kind.drain(..).for_each(|c| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { c.insert(&pool).await }))
                });
                service.drain(..).for_each(|s| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { s.insert(&pool).await }))
                });
                position.drain(..).for_each(|p| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { p.insert(&pool).await }))
                });
                manufacturer.drain(..).for_each(|m| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { m.insert(&pool).await }))
                });
                person.drain(..).for_each(|p| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { p.insert(&pool).await }))
                });
                supplier.drain(..).for_each(|s| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { s.insert(&pool).await }))
                });

                try_join_all(tasks).await?;
            }

            // First group tables (Sync groups)
            {
                println!("Inserting first group tables data");
                let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                    Vec::with_capacity(labor_contract.len());

                labor_contract.drain(..).for_each(|lc| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { lc.insert(&pool).await }))
                });
                staff.drain(..).for_each(|lc| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { lc.insert(&pool).await }))
                });
                try_join_all(tasks).await?;
            }

            // Second group tables
            {
                println!("Inserting second group tables data");
                let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                    Vec::with_capacity(staff.len());

                staff.drain(..).for_each(|s| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { s.insert(&pool).await }))
                });
                try_join_all(tasks).await?;
            }

            // Third group tables
            {
                println!("Inserting third group tables data");
                let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                    Vec::with_capacity(account.len());

                account.drain(..).for_each(|a| {
                    let pool = Arc::clone(&pool);
                    tasks.push(Box::pin(async move { a.insert(&pool).await }))
                });
                try_join_all(tasks).await?;
            }
        }
    }

    Ok(())
}

/// Creates connection pool to database
async fn open_pool(uri: DatabaseUri, opts: &PoolOpts) -> Result<PgPool, Error> {
    let options: PgConnectOptions = uri.inner.as_str().parse()?;

    // Connecting to database
    let pool = PoolOptions::new()
        .min_connections(opts.min_conns)
        .max_connections(opts.max_conns)
        .connect_with(
            options
                .application_name("CW-CLI")
                .ssl_mode(PgSslMode::Prefer),
        )
        .await?;

    Ok(pool)
}

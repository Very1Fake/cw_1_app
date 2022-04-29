use std::{fs::File, io::Write, sync::Arc};

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
use serde_json::{json, to_string};

use crate::opt::{Command, Database, DatabaseOpt, DatabaseUri, Generate, Opt, PoolOpts, SslMode};

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
        Command::Generate(command) => {
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
                mut phone_model,
                mut staff,
                mut component,
                mut phone,
                mut account,
                mut supply_contract,
                mut order,
                mut supply,
                mut warehouse,
                mut service_phone_model,
                mut warehouse_supply,
                mut order_service,
                mut order_warehouse,
            ) = gen.gen_full();
            println!(" : Done");

            match command {
                Generate::Push { uri } => {
                    let pool = Arc::new(open_pool(uri, &opt.pool_opts).await?);

                    // Low-level tables
                    {
                        println!("Inserting low-level tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(
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
                            Vec::with_capacity(labor_contract.len() + phone_model.len());

                        labor_contract.drain(..).for_each(|lc| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { lc.insert(&pool).await }))
                        });
                        phone_model.drain(..).for_each(|pm| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { pm.insert(&pool).await }))
                        });
                        try_join_all(tasks).await?;
                    }

                    // Second group tables
                    {
                        println!("Inserting second group tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(staff.len() + component.len() + phone.len());

                        staff.drain(..).for_each(|s| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { s.insert(&pool).await }))
                        });
                        component.drain(..).for_each(|c| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { c.insert(&pool).await }))
                        });
                        phone.drain(..).for_each(|p| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { p.insert(&pool).await }))
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

                    // Fourth group tables
                    {
                        println!("Inserting fourth group tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(order.len() + supply_contract.len());

                        order.drain(..).for_each(|o| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { o.insert(&pool).await }))
                        });
                        supply_contract.drain(..).for_each(|s_p| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { s_p.insert(&pool).await }))
                        });
                        try_join_all(tasks).await?;
                    }

                    // Fifth group tables
                    {
                        println!("Inserting fifth group tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(
                                supply.len() + service_phone_model.len() + warehouse.len(),
                            );

                        supply.drain(..).for_each(|s| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { s.insert(&pool).await }))
                        });
                        service_phone_model.drain(..).for_each(|s_pm| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { s_pm.insert(&pool).await }))
                        });
                        warehouse.drain(..).for_each(|w| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { w.insert(&pool).await }))
                        });
                        try_join_all(tasks).await?;
                    }

                    // Sixth group tables
                    {
                        println!("Inserting sixth group tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(order_service.len() + warehouse_supply.len());

                        order_service.drain(..).for_each(|o_s| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { o_s.insert(&pool).await }))
                        });
                        warehouse_supply.drain(..).for_each(|w_s| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { w_s.insert(&pool).await }))
                        });
                        try_join_all(tasks).await?;
                    }

                    // Seventh group tables
                    {
                        println!("Inserting seventh group tables data");
                        let mut tasks: Vec<BoxFuture<Result<PgQueryResult, Error>>> =
                            Vec::with_capacity(order_warehouse.len());

                        order_warehouse.drain(..).for_each(|o_w| {
                            let pool = Arc::clone(&pool);
                            tasks.push(Box::pin(async move { o_w.insert(&pool).await }))
                        });
                        try_join_all(tasks).await?;
                    }
                }
                Generate::Dump { path } => {
                    let mut file = File::options()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(path)?;
                    let dump = json!({
                        "component_kind": component_kind,
                        "service": service,
                        "position": position,
                        "manufacturer": manufacturer,
                        "person": person,
                        "supplier": supplier,
                        "labor_contract": labor_contract,
                        "phone_model": phone_model,
                        "staff": staff,
                        "component": component,
                        "phone": phone,
                        "account": account,
                        "warehouse": warehouse,
                        "order": order,
                        "service_phone_model": service_phone_model,
                        "order_service": order_service,
                        "order_warehouse": order_warehouse,
                    });

                    file.write_all(to_string(&dump)?.as_bytes())?;
                }
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
                .ssl_mode(match uri.ssl_mode {
                    SslMode::Disable => PgSslMode::Disable,
                    SslMode::Allow => PgSslMode::Allow,
                    SslMode::Prefer => PgSslMode::Prefer,
                    SslMode::Require => PgSslMode::Require,
                    SslMode::VerifyCa => PgSslMode::VerifyCa,
                    SslMode::VerifyFull => PgSslMode::VerifyFull,
                }),
        )
        .await?;

    Ok(pool)
}

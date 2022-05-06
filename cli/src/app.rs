use std::{fs::File, io::Write, sync::Arc};

use anyhow::{Context, Result};

use cw_core::{
    extensions::Extension,
    functions::Function,
    generator::Config,
    procedures::Procedure,
    sqlx::{
        pool::PoolOptions,
        postgres::{PgConnectOptions, PgSslMode},
        query, Error, Executor, PgPool,
    },
    tables::Table,
    traits::Insertable,
    triggers::Trigger,
    types::DbType,
    views::View,
};
use futures::future::try_join_all;
use serde_json::{json, to_string};

use crate::opt::{Command, Database, DatabaseOpt, DatabaseUri, Generate, Opt, PoolOpts, SslMode};

pub async fn app(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Database(DatabaseOpt { db, command }) => {
            let pool = open_pool(db, &opt.pool_opts).await?;
            match command {
                Database::Create => {
                    println!("\n- Loading extensions");
                    Extension::create_all(&pool, |(extension, p)| {
                        if p {
                            println!("> Extension '{extension}' has been loaded")
                        }
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating types");
                    DbType::create_all(&pool, |(db_type, result)| {
                        match &result {
                            Ok(_) => println!("> Type '{db_type}' has been created"),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating tables");
                    Table::create_all(&pool, |(table, result)| {
                        match &result {
                            Ok(_) => println!("> Table '{table}' has been created"),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating views");
                    View::create_all(&pool, |(view, result)| {
                        match &result {
                            Ok(_) => println!("> View '{view}' has been created"),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating functions");
                    Function::create_all(&pool, |(function, result)| {
                        match &result {
                            Ok(_) => println!("> Function '{function}' has been created"),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating procedures");
                    Procedure::create_all(&pool, |(procedure, result)| {
                        match &result {
                            Ok(_) => println!("> Procedure '{procedure}' has been created"),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Creating triggers");
                    Trigger::create_all(&pool, |(trigger, table, result)| {
                        match &result {
                            Ok(_) => println!(
                                ">> Trigger '{trigger}' for '{table}' table has been created"
                            ),
                            Err(_) => (),
                        };
                        result
                    })
                    .await?;
                    println!("- Done\n");
                }
                Database::Drop => {
                    print!("\n- Dropping triggers\n");
                    Trigger::drop_all(&pool, |(trigger, table, result)| {
                        match &result {
                            Ok(_) => println!(
                                ">> Trigger '{trigger}' for '{table}' table has been dropped"
                            ),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    print!("\n- Dropping procedures\n");
                    Procedure::drop_all(&pool, |(procedure, result)| {
                        match &result {
                            Ok(_) => println!("> Procedure '{procedure}' has been dropped"),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Dropping functions");
                    Function::drop_all(&pool, |(function, result)| {
                        match &result {
                            Ok(_) => println!("> Function '{function}' has been dropped"),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Dropping views");
                    View::drop_all(&pool, |(view, result)| {
                        match &result {
                            Ok(_) => println!("> View '{view}' has been dropped"),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Dropping tables");
                    Table::drop_all(&pool, |(table, result)| {
                        match &result {
                            Ok(_) => println!("> Table '{table}' has been dropped"),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Dropping types");
                    DbType::drop_all(&pool, |(db_type, result)| {
                        match &result {
                            Ok(_) => println!("> Type '{db_type}' has been dropped"),
                            Err(_) => (),
                        }
                        result
                    })
                    .await?;
                    println!("- Done\n");
                    println!("\n- Unloading extensions");
                    Extension::drop_all(&pool, |(e, p)| {
                        if p {
                            println!("> Extension '{e}' has been unloaded")
                        }
                    })
                    .await?;
                    println!("- Done\n");
                }
                Database::Check { fix } => {
                    println!("\n- Checking extensions");
                    for e in Extension::ALL {
                        print!("> Checking '{e}' extension : ");
                        if e.exists(&pool)
                            .await
                            .context("While checking extension existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Loaded");

                            if fix {
                                print!(">> Trying to load extension '{e}' : ");
                                match pool.execute(query(e.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(_) => {
                                        println!("Failed\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking types");
                    for t in DbType::ALL {
                        print!("> Checking '{t}' type : ");
                        if t.exists(&pool)
                            .await
                            .context("While checking type existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create type '{t}' : ");
                                match pool.execute(query(t.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking tables");
                    for table in Table::ALL {
                        print!("> Checking '{table}' table : ");
                        if table
                            .exists(&pool)
                            .await
                            .context("While checking table existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create table '{table}' : ");
                                match pool.execute(query(table.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking view");
                    for view in View::ALL {
                        print!("> Checking '{view}' view : ");
                        if view
                            .exists(&pool)
                            .await
                            .context("While checking view existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create view '{view}' : ");
                                match pool.execute(query(view.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking function");
                    for function in Function::ALL {
                        print!("> Checking '{function}' function : ");
                        if function
                            .exists(&pool)
                            .await
                            .context("While checking function existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create function '{function}' : ");
                                match pool.execute(query(function.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking procedures");
                    for procedure in Procedure::ALL {
                        print!("> Checking '{procedure}' procedure : ");
                        if procedure
                            .exists(&pool)
                            .await
                            .context("While checking procedure existence")?
                        {
                            println!("OK");
                        } else {
                            println!("Not Exists");

                            if fix {
                                print!(">> Trying to create procedure '{procedure}' : ");
                                match pool.execute(query(procedure.create())).await {
                                    Ok(_) => println!("OK\n"),
                                    Err(err) => {
                                        println!("Failed\n>>\t{err}\n");
                                    }
                                };
                            }
                        }
                    }
                    println!("- Done\n");

                    println!("\n- Checking triggers");
                    for trigger in Trigger::ALL {
                        println!("> Checking '{trigger}' trigger:");
                        for (table, result) in trigger.exists(&pool).await {
                            print!(">> Checking for '{table}' table : ");
                            match result {
                                Ok(ok) => {
                                    if ok {
                                        println!("OK");
                                    } else {
                                        println!("Not Exists");

                                        if fix {
                                            print!(">>> Trying to create trigger for '{table}' table : ");
                                            match query(&trigger.create(table)).execute(&pool).await
                                            {
                                                Ok(_) => println!("OK\n"),
                                                Err(err) => {
                                                    println!("Failed\n>>>\t{err}\n");
                                                }
                                            };
                                        }
                                    }
                                }
                                Err(_) => todo!(),
                            }
                        }
                    }
                    println!("- Done\n");
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

                    fn mapper<T>(obj: T) -> Box<dyn Insertable + Send + Sync>
                    where
                        T: Insertable + Send + Sync + 'static,
                    {
                        Box::new(obj)
                    }

                    async fn insert_group(
                        pool: &Arc<PgPool>,
                        name: &str,
                        objects: impl Iterator<Item = Box<dyn Insertable + Send + Sync>>,
                    ) -> Result<(), Error> {
                        print!("Inserting {name} : ");

                        try_join_all(objects.map(|obj| {
                            let pool = Arc::clone(pool);
                            Box::pin(async move { obj.insert().execute(&*pool).await })
                        }))
                        .await?;

                        println!("Done");

                        Ok(())
                    }

                    insert_group(
                        &pool,
                        "low-level",
                        component_kind
                            .drain(..)
                            .map(mapper)
                            .chain(service.drain(..).map(mapper))
                            .chain(position.drain(..).map(mapper))
                            .chain(manufacturer.drain(..).map(mapper))
                            .chain(person.drain(..).map(mapper))
                            .chain(supplier.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(
                        &pool,
                        "first",
                        labor_contract
                            .drain(..)
                            .map(mapper)
                            .chain(phone_model.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(
                        &pool,
                        "second",
                        staff
                            .drain(..)
                            .map(mapper)
                            .chain(component.drain(..).map(mapper))
                            .chain(phone.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(&pool, "third", account.drain(..).map(mapper)).await?;

                    insert_group(
                        &pool,
                        "fourth",
                        order
                            .drain(..)
                            .map(mapper)
                            .chain(supply_contract.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(
                        &pool,
                        "fifth",
                        supply
                            .drain(..)
                            .map(mapper)
                            .chain(service_phone_model.drain(..).map(mapper))
                            .chain(warehouse.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(
                        &pool,
                        "sixth",
                        order_service
                            .drain(..)
                            .map(mapper)
                            .chain(warehouse_supply.drain(..).map(mapper)),
                    )
                    .await?;

                    insert_group(&pool, "seventh", order_warehouse.drain(..).map(mapper)).await?;
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
        // .after_connect(|conn: &mut PgConnection| {
        //     Box::pin(async move {
        //         conn.execute("SET user = 'cli-tool';").await?;
        //         Ok(())
        //     })
        // })
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

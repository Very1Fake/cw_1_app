use anyhow::{Context, Result};

use cw_core::{
    extensions::Extension,
    sqlx::{
        postgres::{PgConnectOptions, PgSslMode},
        query, Error, Executor, PgPool,
    },
    tables::{
        Account, Component, ComponentKind, LaborContract, Manufacturer, Order, OrderService,
        OrderWarehouse, Person, Phone, PhoneModel, Position, Service, ServicePhoneModel, Staff,
        Supplier, Supply, SupplyContract, Table, Warehouse, WarehouseSupply,
    },
    types::{
        AccountRole, AccountStatus, Color, ContractStatus, DbType, MetaTime, OrderStatus,
        StaffStatus, SupplyStatus,
    },
    BigDecimal,
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
            let pool = open_pool(uri).await?;

            println!("Inserting Person");

            let person = Person::new_auto(
                String::from("Mike"),
                None,
                String::from("Smith"),
                String::from("mike_smith@gmail.com"),
                String::from("79876543211"),
            );
            person.insert(&pool).await?;

            println!("Inserting Supplier");

            let supplier = Supplier::new_auto(
                String::from("FoxCon"),
                String::from("1234568901234568790123456789012"),
                String::from("FT123FT232"),
                String::from("China, Beijing, ..."),
                String::from("CH"),
            );
            supplier.insert(&pool).await?;

            println!("Inserting Manufacturer");

            let manufacturer = Manufacturer::new_auto(String::from("Apple"), String::from("US"));
            manufacturer.insert(&pool).await?;

            println!("Inserting Service");

            let service = Service::new_auto(String::from("Replace: display glass"), None);
            service.insert(&pool).await?;

            println!("Inserting Position");

            let position = Position::new_auto(
                String::from("Administrator"),
                None,
                BigDecimal::from(100_000_i32),
            );
            position.insert(&pool).await?;

            println!("Inserting ComponentKind");

            let component_kind = ComponentKind::new_auto(
                String::from("Display Glass"),
                Some(String::from("Glass that is placed on top of display")),
            );
            component_kind.insert(&pool).await?;

            println!("Inserting LaborContract");

            let labor_contract = LaborContract::new_auto(
                person.uuid,
                String::from("1234567856"),
                ContractStatus::Active,
                Some(MetaTime::now()),
            );
            labor_contract.insert(&pool).await?;

            println!("Inserting Staff");

            let staff = Staff::new_auto(labor_contract.uuid, position.uuid, StaffStatus::Working);
            staff.insert(&pool).await?;

            println!("Inserting Account");

            let account = Account::new_auto(
                staff.uuid,
                String::from("temp_user"),
                String::from(""),
                AccountRole::Admin,
                AccountStatus::Active,
            );
            account.insert(&pool).await?;

            println!("Inserting SupplyContract");

            let supply_contract = SupplyContract::new_auto(
                supplier.uuid,
                staff.uuid,
                ContractStatus::Active,
                Some(MetaTime::now()),
            );
            supply_contract.insert(&pool).await?;

            println!("Inserting PhoneModel");

            let phone_model =
                PhoneModel::new_auto(String::from("IPhone X"), None, manufacturer.uuid);
            phone_model.insert(&pool).await?;

            println!("Inserting Phone");

            let phone = Phone::new_auto(
                person.uuid,
                String::from("356741082068981"),
                "8e:c3:0d:74:a7:d5".parse()?,
                "e7:bd:8d:06:eb:5b".parse()?,
                phone_model.uuid,
                Color::White,
            );
            phone.insert(&pool).await?;

            println!("Inserting Component");

            let component = Component::new_auto(
                String::from("Display Glass for IPhone X"),
                component_kind.uuid,
                phone_model.uuid,
                manufacturer.uuid,
            );
            component.insert(&pool).await?;

            println!("Inserting ServicePhoneModel");

            ServicePhoneModel::new_auto(
                service.uuid,
                phone_model.uuid,
                BigDecimal::from(10_000_i32),
            )
            .insert(&pool)
            .await?;

            println!("Inserting Supply");

            let supply = Supply::new_auto(
                supply_contract.uuid,
                staff.uuid,
                SupplyStatus::Delivered,
                Some(MetaTime::now()),
            );
            supply.insert(&pool).await?;

            println!("Inserting Warehouse");

            let warehouse = Warehouse::new_auto(
                component.uuid,
                supplier.uuid,
                BigDecimal::from(15_000_i32),
                10,
            );
            warehouse.insert(&pool).await?;

            println!("Inserting WarehouseSupply");

            WarehouseSupply::new(warehouse.uuid, supply.uuid, 10, MetaTime::now())
                .insert(&pool)
                .await?;

            println!("Inserting Order");

            let order = Order::new_auto(
                person.uuid,
                phone.uuid,
                staff.uuid,
                staff.uuid,
                OrderStatus::Complete,
            );
            order.insert(&pool).await?;

            println!("Inserting OrderWarehouse");

            OrderWarehouse::new(order.uuid, warehouse.uuid, 1, BigDecimal::from(15_000_i32))
                .insert(&pool)
                .await?;

            println!("Inserting OrderService");

            OrderService::new(order.uuid, service.uuid, BigDecimal::from(10_100_i32))
                .insert(&pool)
                .await?;
        }
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

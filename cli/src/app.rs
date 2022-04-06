use anyhow::Result;

use cw_core::{
    extensions::{ex_create_all, ex_drop_all},
    sqlx::PgPool,
    tables::{tb_create_all, tb_drop_all},
    types::{tp_create_all, tp_drop_all},
};

use crate::opt::{Op, Opt};

pub async fn app(opt: Opt, pool: PgPool) -> Result<()> {
    match opt.op {
        Op::Create => {
            println!("- Loading extensions");
            ex_create_all(&pool, |(e, r)| {
                match &r {
                    Ok(_) => println!("> Extension '{e}' has been loaded"),
                    Err(_) => (),
                };
                r
            })
            .await?;
            println!("- Loading extensions: Done!");
            println!("- Creating types");
            tp_create_all(&pool, |(t, r)| {
                match &r {
                    Ok(_) => println!("> Type '{t}' has been created"),
                    Err(_) => (),
                };
                r
            })
            .await?;
            println!("- Creating types: Done!");
            println!("- Creating tables");
            tb_create_all(&pool, |(t, r)| {
                match &r {
                    Ok(_) => println!("> Table '{t}' has been created"),
                    Err(_) => (),
                };
                r
            })
            .await?;
            println!("- Creating tables: Done!");
        }
        Op::Drop => {
            println!("- Dropping tables");
            tb_drop_all(&pool, |(t, r)| {
                match &r {
                    Ok(_) => println!("> Table '{t}' has been dropped"),
                    Err(_) => (),
                }
                r
            })
            .await?;
            println!("- Dropping tables: Done!");
            println!("- Dropping types");
            tp_drop_all(&pool, |(t, r)| {
                match &r {
                    Ok(_) => println!("> Type '{t}' has been dropped"),
                    Err(_) => (),
                }
                r
            })
            .await?;
            println!("- Dropping types: Done!");
            println!("- Unloading extensions");
            ex_drop_all(&pool, |(e, r)| {
                match &r {
                    Ok(_) => println!("> Extension '{e}' has been unloaded"),
                    Err(_) => (),
                }
                r
            })
            .await?;
            println!("- Unloading extensions: Done!");
        }
        Op::Check => todo!(),
    }

    Ok(())
}

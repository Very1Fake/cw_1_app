use std::ops::RangeInclusive;

use fake::{
    faker::{
        address::raw::*,
        company::raw::*,
        finance::raw::*,
        internet::raw::{FreeEmail, MACAddress, Password, Username},
        name::raw::*,
        phone_number::raw::*,
    },
    locales::{EN, ZH_CN, ZH_TW},
    Fake,
};
use rand::{
    distributions::{Bernoulli, Slice, WeightedIndex},
    prelude::{Distribution, ThreadRng},
    thread_rng, Rng,
};
use sqlx::types::BigDecimal;

use crate::{
    tables::{
        Account, Component, ComponentKind, LaborContract, Manufacturer, Order, OrderService,
        OrderWarehouse, Person, Phone, PhoneModel, Position, Service, ServicePhoneModel, Staff,
        Supplier, Supply, SupplyContract, Warehouse, WarehouseSupply,
    },
    types::{
        AccountRole, AccountStatus, Color, ContractStatus, MetaTime, OrderStatus, StaffStatus,
        SupplyStatus,
    },
    utils::password_hash,
};

pub type FullPack = (
    Vec<ComponentKind>,
    Vec<Service>,
    Vec<Position>,
    Vec<Manufacturer>,
    Vec<Person>,
    Vec<Supplier>,
    Vec<LaborContract>,
    Vec<PhoneModel>,
    Vec<Staff>,
    Vec<Component>,
    Vec<Phone>,
    Vec<Account>,
    Vec<SupplyContract>,
    Vec<Order>,
    Vec<Supply>,
    Vec<Warehouse>,
    Vec<ServicePhoneModel>,
    Vec<WarehouseSupply>,
    Vec<OrderService>,
    Vec<OrderWarehouse>,
);

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const IBAN_ID_RANGE: RangeInclusive<i128> = 0..=9999999999999999999999999999;

#[derive(Debug)]
pub struct Config {
    pub person_count: usize,
    pub supplier_count: usize,
    pub position_salary_scatter: f32,

    // LaborContract
    pub labor_contract_count: usize,
    pub labor_contract_weights: [u8; 5],

    // Staff
    pub staff_vacation_chance: f64,

    // Account
    pub account_status_weights: [u8; 3],

    // Supply Contract
    pub supply_contract_chance: f64,
    pub supply_contract_count: usize,

    // Phone
    pub phone_count: Vec<u8>,

    // Warehouse
    pub warehouse_variations: u32,
    pub warehouse_stock: (u16, u16),
    pub warehouse_item_price_scatter: f64,

    // Order
    pub order_count: usize,
    pub order_not_owner_chance: f64,

    // Supply
    pub supply_count: usize,
}

impl Config {
    pub fn gen_person(&self) -> Vec<Person> {
        let mut store = Vec::with_capacity(self.person_count);

        for _ in 0..self.person_count {
            store.push(Person::new_auto(
                FirstName(EN).fake(),
                None,
                LastName(EN).fake(),
                FreeEmail(EN).fake(),
                CellNumber(EN).fake(),
            ))
        }

        store
    }

    pub fn gen_positions(&self) -> Vec<Position> {
        let mut rng = thread_rng();
        Position::SAMPLES
            .iter()
            .map(|(name, salary, ..)| {
                let salary = *salary as f32;
                let scatter = salary * self.position_salary_scatter;
                Position::new_auto(
                    name.to_string(),
                    None,
                    BigDecimal::from(rng.gen_range((salary - scatter)..(salary + scatter)) as i64),
                )
            })
            .collect()
    }

    pub fn gen_manufacturer(&self) -> Vec<Manufacturer> {
        Manufacturer::SAMPLES
            .iter()
            .map(|(name, country)| Manufacturer::new_auto(name.to_string(), country.to_string()))
            .collect()
    }

    pub fn gen_component_kind(&self) -> Vec<ComponentKind> {
        ComponentKind::SAMPLES
            .iter()
            .map(|(name, desc, _)| {
                ComponentKind::new_auto(name.to_string(), desc.map(|val| val.to_string()))
            })
            .collect()
    }

    pub fn gen_service(&self) -> Vec<Service> {
        Service::SAMPLES
            .iter()
            .map(|(name, desc, ..)| {
                Service::new_auto(name.to_string(), desc.map(|val| val.to_string()))
            })
            .collect()
    }

    pub fn gen_supplier(&self) -> Vec<Supplier> {
        let mut store = Vec::with_capacity(self.supplier_count);
        let mut rng = thread_rng();

        for _ in 0..self.supplier_count {
            let country: usize = rng.gen_range(0..3);
            let name = match country {
                // EN
                0 => CompanyName(EN).fake(),
                // ZH_CN
                1 => CompanyName(ZH_CN).fake(),
                // ZH_TW
                2 => CompanyName(ZH_TW).fake(),
                _ => unreachable!(),
            };
            // Invalid Iban
            let iban = {
                let id = rng.gen_range(IBAN_ID_RANGE);
                format!("{}{:030}", country_match(country), id)
            };
            let swift = match country {
                // EN
                0 => Bic(EN).fake(),
                // ZH_CN
                1 => Bic(ZH_CN).fake(),
                // ZH_TW
                2 => Bic(ZH_TW).fake(),
                _ => unreachable!(),
            };
            let (code, zip, state, city, street, secondary): (
                &str,
                String,
                String,
                String,
                String,
                String,
            ) = match country {
                // EN
                0 => (
                    "USA",
                    ZipCode(EN).fake(),
                    StateAbbr(EN).fake(),
                    CityName(EN).fake(),
                    StreetName(EN).fake(),
                    SecondaryAddress(EN).fake(),
                ),
                // ZH_CN
                1 => (
                    "CN",
                    ZipCode(ZH_CN).fake(),
                    StateAbbr(ZH_CN).fake(),
                    CityName(ZH_CN).fake(),
                    StreetName(ZH_CN).fake(),
                    SecondaryAddress(ZH_CN).fake(),
                ),
                // ZH_TW
                2 => (
                    "TW",
                    ZipCode(ZH_TW).fake(),
                    StateAbbr(ZH_TW).fake(),
                    CityName(ZH_TW).fake(),
                    StreetName(ZH_TW).fake(),
                    SecondaryAddress(ZH_TW).fake(),
                ),
                _ => unreachable!(),
            };
            let address = format!(
                "{} {} {}, {}, {}, {}",
                code, zip, state, city, street, secondary
            );

            store.push(Supplier::new_auto(
                name,
                iban,
                swift,
                address,
                country_match(country).to_string(),
            ))
        }

        store
    }

    /// Generate labor contracts from given list of persons
    /// # Panics
    /// This function will panic if [person_store] length is less than [labor_contract_count]
    pub fn gen_labor_contract(&self, people: &[Person]) -> Vec<LaborContract> {
        if people.len() < self.labor_contract_count {
            panic!("Person count less than count of labor contracts");
        }

        let mut rng = thread_rng();
        let status_weights = WeightedIndex::new(self.labor_contract_weights).unwrap();
        let mut labor_contract_store = Vec::with_capacity(self.labor_contract_count);

        for _ in 0..self.labor_contract_count {
            loop {
                // FIX: Inf loop
                let p = people.get(rng.gen_range(0..people.len())).unwrap();
                if !labor_contract_store
                    .iter()
                    .any(|c: &LaborContract| c.person == p.uuid)
                {
                    let status = ContractStatus::ALL[status_weights.sample(&mut rng)];
                    labor_contract_store.push(LaborContract::new_auto(
                        p.uuid,
                        rand_string(&mut rng, 10),
                        status,
                        match status {
                            ContractStatus::Review | ContractStatus::Negotiation => None,
                            ContractStatus::Active
                            | ContractStatus::Expired
                            | ContractStatus::Void
                            | ContractStatus::Rejected => Some(MetaTime::now()),
                        },
                    ));
                    break;
                }
            }
        }

        labor_contract_store
    }

    pub fn gen_phone_model(&self, manufacturers: &[Manufacturer]) -> Vec<PhoneModel> {
        PhoneModel::SAMPLES
            .into_iter()
            .map(|(n, d, m, _)| {
                PhoneModel::new_auto(
                    n.to_string(),
                    if d.is_empty() {
                        None
                    } else {
                        Some(d.to_string())
                    },
                    match manufacturers.iter().find_map(|manuf| {
                        if manuf.name == m {
                            Some(manuf.uuid)
                        } else {
                            None
                        }
                    }) {
                        Some(uuid) => uuid,
                        None => panic!("Manufacturer '{m}' not found for '{n}' model"),
                    },
                )
            })
            .collect()
    }

    pub fn gen_staff(
        &self,
        labor_contracts: &[LaborContract],
        positions: &[Position],
    ) -> Vec<Staff> {
        let mut rng = thread_rng();
        let vacation_weights = Bernoulli::new(self.staff_vacation_chance).unwrap();
        let position_weights = WeightedIndex::new(
            Position::SAMPLES
                .into_iter()
                .map(|(.., c, _)| c)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        labor_contracts
            .iter()
            .filter(|lc| {
                !matches!(
                    lc.status,
                    ContractStatus::Review | ContractStatus::Negotiation
                )
            })
            .map(|lc| {
                Staff::new_auto(
                    lc.uuid,
                    positions[position_weights.sample(&mut rng)].uuid,
                    match lc.status {
                        ContractStatus::Active => {
                            if vacation_weights.sample(&mut rng) {
                                StaffStatus::OnVacation
                            } else {
                                StaffStatus::Working
                            }
                        }
                        ContractStatus::Expired => StaffStatus::Suspended,
                        ContractStatus::Void => StaffStatus::Fired,
                        _ => unreachable!(),
                    },
                )
            })
            .collect()
    }

    pub fn gen_component(
        &self,
        manufacturers: &[Manufacturer],
        component_kinds: &[ComponentKind],
        phone_models: &[PhoneModel],
    ) -> Vec<Component> {
        Component::SAMPLES
            .into_iter()
            .map(|(n, k, pm, m)| {
                Component::new_auto(
                    n.to_string(),
                    match component_kinds.iter().find_map(|comp| {
                        if comp.name == k {
                            Some(comp.uuid)
                        } else {
                            None
                        }
                    }) {
                        Some(uuid) => uuid,
                        None => panic!("Component kind '{k}' not found for '{n}' component"),
                    },
                    match phone_models.iter().find_map(|model| {
                        if model.name == pm {
                            Some(model.uuid)
                        } else {
                            None
                        }
                    }) {
                        Some(uuid) => uuid,
                        None => panic!("Phone model '{pm}' not found for '{n}' component"),
                    },
                    match manufacturers.iter().find_map(|manuf| {
                        if manuf.name == m {
                            Some(manuf.uuid)
                        } else {
                            None
                        }
                    }) {
                        Some(uuid) => uuid,
                        None => panic!("Manufacturer '{m}' not found for '{n}' component"),
                    },
                )
            })
            .collect()
    }

    pub fn gen_phone(&self, people: &[Person], phone_models: &[PhoneModel]) -> Vec<Phone> {
        let mut rng = thread_rng();
        let phone_count_chances = WeightedIndex::new(&self.phone_count).unwrap();
        people
            .iter()
            .flat_map(|p| {
                let count = phone_count_chances.sample(&mut rng);
                let mut phones = Vec::with_capacity(count);

                for _ in 0..=count {
                    phones.push(Phone::new_auto(
                        p.uuid,
                        rand_string(&mut rng, 17),
                        MACAddress(EN).fake::<String>().parse().unwrap(),
                        MACAddress(EN).fake::<String>().parse().unwrap(),
                        phone_models[rng.gen_range(0..phone_models.len())].uuid,
                        Color::ALL[rng.gen_range(0..Color::ALL.len())],
                    ));
                }

                phones
            })
            .collect()
    }

    pub fn gen_account(&self, staff: &[Staff], positions: &[Position]) -> Vec<Account> {
        let account_weights = WeightedIndex::new(self.account_status_weights).unwrap();
        let mut rng = thread_rng();

        staff
            .iter()
            .map(|s| {
                Account::new_auto(
                    s.uuid,
                    Username(EN).fake(),
                    password_hash(Password(EN, 8..33).fake::<String>().as_bytes()).unwrap(),
                    positions
                        .iter()
                        .find_map(|p| {
                            if p.uuid == s.position {
                                Some(
                                    Position::SAMPLES
                                        .into_iter()
                                        .find_map(
                                            |(n, .., r)| {
                                                if p.name == n {
                                                    Some(r)
                                                } else {
                                                    None
                                                }
                                            },
                                        )
                                        .unwrap(),
                                )
                            } else {
                                None
                            }
                        })
                        .unwrap(),
                    AccountStatus::ALL[account_weights.sample(&mut rng)],
                )
            })
            .collect()
    }

    pub fn gen_supply_contract(
        &self,
        suppliers: &[Supplier],
        staff: &[Staff],
        accounts: &[Account],
    ) -> Vec<SupplyContract> {
        let mut rng = thread_rng();
        let managers = &staff
            .iter()
            .filter_map(|staff| {
                match accounts
                    .iter()
                    .find(|account| account.staff == staff.uuid)
                    .unwrap()
                    .role
                {
                    AccountRole::Manager => Some(staff.uuid),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if managers.is_empty() {
            panic!("There is no managers on staff");
        }

        let old_statuses = Slice::new(&[
            ContractStatus::Expired,
            ContractStatus::Void,
            ContractStatus::Rejected,
        ])
        .unwrap();
        let new_statuses = Slice::new(&[
            ContractStatus::Review,
            ContractStatus::Negotiation,
            ContractStatus::Active,
        ])
        .unwrap();

        suppliers
            .iter()
            .flat_map(|supplier| {
                if rng.gen_bool(self.supply_contract_chance) {
                    let count = rng.gen_range(0..=self.supply_contract_count);
                    let mut contracts = Vec::with_capacity(count);

                    for i in 0..count {
                        let status = if i + 1 == count {
                            new_statuses.sample(&mut rng)
                        } else {
                            old_statuses.sample(&mut rng)
                        };
                        contracts.push(SupplyContract::new_auto(
                            supplier.uuid,
                            managers[rng.gen_range(0..managers.len())],
                            *status,
                            match status {
                                ContractStatus::Review
                                | ContractStatus::Negotiation
                                | ContractStatus::Rejected => None,
                                ContractStatus::Active
                                | ContractStatus::Expired
                                | ContractStatus::Void => Some(MetaTime::now()),
                            },
                        ));
                    }

                    contracts
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    pub fn gen_supply(
        &self,
        supply_contracts: &[SupplyContract],
        staff: &[Staff],
        accounts: &[Account],
    ) -> Vec<Supply> {
        let mut rng = thread_rng();
        let workers = &staff
            .iter()
            .filter_map(|staff| {
                match accounts
                    .iter()
                    .find(|account| account.staff == staff.uuid)
                    .unwrap()
                    .role
                {
                    AccountRole::WarehouseWorker => Some(staff.uuid),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if workers.is_empty() {
            panic!("There is no warehouse workers on staff");
        }

        let old_statuses = Slice::new(&[
            SupplyStatus::Delivered,
            SupplyStatus::Failed,
            SupplyStatus::Rejected,
        ])
        .unwrap();
        let new_statuses = Slice::new(&[
            SupplyStatus::Review,
            SupplyStatus::Negotiation,
            SupplyStatus::Signed,
            SupplyStatus::Paid,
            SupplyStatus::Dispatched,
        ])
        .unwrap();

        supply_contracts
            .iter()
            .filter(|supply_contract| {
                matches!(
                    supply_contract.status,
                    ContractStatus::Active | ContractStatus::Expired | ContractStatus::Void
                )
            })
            .flat_map(|supply_contract| {
                let count = rng.gen_range(0..=self.supply_count);
                let mut supplies = Vec::with_capacity(count);

                for i in 0..count {
                    let status = if i + 1 == count {
                        new_statuses.sample(&mut rng)
                    } else {
                        old_statuses.sample(&mut rng)
                    };
                    supplies.push(Supply::new_auto(
                        supply_contract.uuid,
                        workers[rng.gen_range(0..workers.len())],
                        *status,
                        match status {
                            SupplyStatus::Review
                            | SupplyStatus::Negotiation
                            | SupplyStatus::Rejected => None,
                            SupplyStatus::Signed
                            | SupplyStatus::Paid
                            | SupplyStatus::Dispatched
                            | SupplyStatus::Delivered
                            | SupplyStatus::Failed => Some(MetaTime::now()),
                        },
                    ))
                }

                supplies
            })
            .collect()
    }

    pub fn gen_warehouse(
        &self,
        components: &[Component],
        component_kinds: &[ComponentKind],
        supply_contracts: &[SupplyContract],
    ) -> Vec<Warehouse> {
        let mut rng = thread_rng();
        let mut suppliers = supply_contracts
            .iter()
            .filter_map(|contract| match contract.status {
                ContractStatus::Review => None,
                ContractStatus::Negotiation => None,
                ContractStatus::Active | ContractStatus::Expired | ContractStatus::Void => {
                    Some(contract.supplier)
                }
                ContractStatus::Rejected => None,
            })
            .collect::<Vec<_>>();
        suppliers.dedup();

        components
            .iter()
            .flat_map(|c| {
                let supp_count = rng.gen_range(0..self.warehouse_variations);
                let mut items = Vec::with_capacity(supp_count as usize);

                for _ in 0..=supp_count {
                    let supp = suppliers[rng.gen_range(0..suppliers.len())];
                    if items.iter().any(|i: &Warehouse| i.supplier == supp) {
                        // FIX: Try until found
                        continue;
                    }

                    items.push(Warehouse::new_auto(
                        c.uuid,
                        supp,
                        component_kinds
                            .iter()
                            .find_map(|k| {
                                if k.uuid == c.kind {
                                    Some(
                                        ComponentKind::SAMPLES
                                            .into_iter()
                                            .find_map(|(n, _, p)| {
                                                if n == k.name {
                                                    Some(BigDecimal::from(
                                                        rng.gen_range(
                                                            (p - self.warehouse_item_price_scatter)
                                                                ..(p + self
                                                                    .warehouse_item_price_scatter),
                                                        )
                                                            as i64,
                                                    ))
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap(),
                                    )
                                } else {
                                    None
                                }
                            })
                            .unwrap(),
                        rng.gen_range(self.warehouse_stock.0..self.warehouse_stock.1) as i32,
                    ));
                }

                items
            })
            .collect()
    }

    pub fn gen_service_phone_model(
        &self,
        services: &[Service],
        phone_models: &[PhoneModel],
    ) -> Vec<ServicePhoneModel> {
        services
            .iter()
            .flat_map(|s| {
                phone_models
                    .iter()
                    .map(|pm| {
                        ServicePhoneModel::new_auto(
                            s.uuid,
                            pm.uuid,
                            Service::SAMPLES
                                .into_iter()
                                .find_map(|(n, _, p, _)| {
                                    if n == s.name {
                                        Some(
                                            PhoneModel::SAMPLES
                                                .into_iter()
                                                .find_map(|(pm_n, _, _, coef)| {
                                                    if pm_n == pm.name {
                                                        Some(BigDecimal::from((p * coef) as i64))
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .unwrap(),
                                        )
                                    } else {
                                        None
                                    }
                                })
                                .unwrap(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn gen_warehouse_supply(
        &self,
        warehouse: &[Warehouse],
        supplies: &[Supply],
        supply_contracts: &[SupplyContract],
    ) -> Vec<WarehouseSupply> {
        supplies
            .iter()
            .flat_map(|supply| {
                let supplier = supply_contracts
                    .iter()
                    .find(|contract| contract.uuid == supply.contract)
                    .unwrap()
                    .supplier;
                // Supply contracts from the supplier
                let contracts = supply_contracts
                    .iter()
                    .filter_map(|contract| {
                        if contract.supplier == supplier {
                            Some(contract.uuid)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                // Supplies for the supply contract
                let supplies = supplies
                    .iter()
                    .filter_map(|supp| {
                        if contracts.contains(&supp.contract) {
                            Some(supp.uuid)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                // Items in warehouse from the supplier
                let items = warehouse
                    .iter()
                    .filter_map(|item| {
                        if item.supplier == supplier {
                            Some((item.uuid, item.amount))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let step = {
                    let s = items.len() / supplies.len();

                    if s == 0 {
                        1
                    } else {
                        s
                    }
                };
                let pos = supplies.iter().position(|&e| e == supply.uuid).unwrap();

                let mut supply_items = Vec::with_capacity(step);
                let mut items = items.chunks(step).skip(pos).peekable();

                if let Some(chunk) = items.next() {
                    for (item, amount) in chunk {
                        supply_items.push(WarehouseSupply::new(
                            *item,
                            supply.uuid,
                            *amount,
                            supply.meta.updated,
                        ));
                    }

                    match items.peek() {
                        Some(chunk) if chunk.len() != step => {
                            for (item, amount) in items.next().expect("while peeking") {
                                supply_items.push(WarehouseSupply::new(
                                    *item,
                                    supply.uuid,
                                    *amount,
                                    supply.meta.updated,
                                ));
                            }
                        }
                        _ => (),
                    }
                }

                supply_items
            })
            .collect()
    }

    pub fn gen_order(
        &self,
        people: &[Person],
        staff: &[Staff],
        accounts: &[Account],
        phones: &[Phone],
    ) -> Vec<Order> {
        let mut rng = thread_rng();
        let servicemen = &staff
            .iter()
            .filter_map(|staff| {
                match accounts
                    .iter()
                    .find(|account| account.staff == staff.uuid)
                    .unwrap()
                    .role
                {
                    AccountRole::Serviceman => Some(staff.uuid),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if servicemen.is_empty() {
            panic!("There is no servicemen on staff");
        }

        let shopmen = &staff
            .iter()
            .filter_map(|staff| {
                match accounts
                    .iter()
                    .find(|account| account.staff == staff.uuid)
                    .unwrap()
                    .role
                {
                    AccountRole::Shopman => Some(staff.uuid),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if shopmen.is_empty() {
            panic!("There is no shopmen on staff");
        }

        phones
            .iter()
            .flat_map(|phone| {
                let count = rng.gen_range(0..self.order_count);
                let mut orders = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    orders.push(Order::new_auto(
                        people.iter().find(|c| c.uuid == phone.person).unwrap().uuid,
                        phone.uuid,
                        servicemen[rng.gen_range(0..servicemen.len())],
                        shopmen[rng.gen_range(0..shopmen.len())],
                        OrderStatus::ALL[rng.gen_range(0..OrderStatus::ALL.len())],
                    ))
                }

                orders
            })
            .collect()
    }

    pub fn gen_order_service(
        &self,
        orders: &[Order],
        phones: &[Phone],
        phone_models: &[PhoneModel],
        service_phone_model: &[ServicePhoneModel],
    ) -> Vec<OrderService> {
        let mut rng = thread_rng();
        orders
            .iter()
            .map(|order| {
                let phone = phones
                    .iter()
                    .find(|phone| phone.uuid == order.phone)
                    .unwrap();
                let phone_model = phone_models
                    .iter()
                    .find(|phone_model| phone_model.uuid == phone.model)
                    .unwrap();
                let spm_list: Vec<&ServicePhoneModel> = service_phone_model
                    .iter()
                    .filter(|spm| spm.phone_model == phone_model.uuid)
                    .collect();

                if spm_list.is_empty() {
                    unreachable!(
                        "No Service-PhoneModel relations has been found for '{}' phone model",
                        phone_model.name
                    );
                } else {
                    let spm = spm_list[rng.gen_range(0..spm_list.len())];

                    OrderService::new(order.uuid, spm.service, spm.price)
                }
            })
            .collect()
    }

    pub fn gen_order_warehouse(
        &self,
        orders: &[Order],
        order_service: &[OrderService],
        services: &[Service],
        component_kinds: &[ComponentKind],
        phones: &[Phone],
        phone_models: &[PhoneModel],
        components: &[Component],
        warehouse: &[Warehouse],
    ) -> Vec<OrderWarehouse> {
        let mut rng = thread_rng();
        orders
            .iter()
            .map(|order| {
                let order_service = order_service
                    .iter()
                    .find(|order_service| order_service.order == order.uuid)
                    .unwrap();
                let service = services
                    .iter()
                    .find(|service| service.uuid == order_service.service)
                    .unwrap();
                let component_hint = Service::SAMPLES
                    .into_iter()
                    .find(|sample| sample.0 == service.name)
                    .unwrap()
                    .3;
                let component_kind = component_kinds
                    .iter()
                    .find(|component_kind| component_kind.name == component_hint)
                    .unwrap();
                let phone = phones
                    .iter()
                    .find(|phone| phone.uuid == order.phone)
                    .unwrap();
                let phone_model = phone_models
                    .iter()
                    .find(|phone_model| phone_model.uuid == phone.model)
                    .unwrap();
                let component = components
                    .iter()
                    .find(|component| {
                        component.kind == component_kind.uuid
                            && component.phone_model == phone_model.uuid
                    })
                    .unwrap();
                let items: Vec<&Warehouse> = warehouse
                    .iter()
                    .filter(|item| item.component == component.uuid)
                    .collect();
                let item = items[rng.gen_range(0..items.len())]; // TODO: Decrease if amount is positive

                OrderWarehouse::new(order.uuid, item.uuid, 1, item.price)
            })
            .collect()
    }

    pub fn gen_full(&self) -> FullPack {
        // Low-level
        let component_kind_store = self.gen_component_kind();
        let service_store = self.gen_service();
        let position_store = self.gen_positions();
        let manufacturer_store = self.gen_manufacturer();
        let person_store = self.gen_person();
        let supplier_store = self.gen_supplier();

        // First group
        let labor_contract_store = self.gen_labor_contract(&person_store);
        let phone_model_store = self.gen_phone_model(&manufacturer_store);

        // Second group
        let staff_store = self.gen_staff(&labor_contract_store, &position_store);
        let component_store = self.gen_component(
            &manufacturer_store,
            &component_kind_store,
            &phone_model_store,
        );
        let phone_store = self.gen_phone(&person_store, &phone_model_store);

        // Third group
        let account_store = self.gen_account(&staff_store, &position_store);

        // Fourth group
        let order_store = self.gen_order(&person_store, &staff_store, &account_store, &phone_store);
        let supply_contract_store =
            self.gen_supply_contract(&supplier_store, &staff_store, &account_store);

        // Fifth group
        let supply_store = self.gen_supply(&supply_contract_store, &staff_store, &account_store);
        let service_phone_model_store =
            self.gen_service_phone_model(&service_store, &phone_model_store);
        let warehouse_store = self.gen_warehouse(
            &component_store,
            &component_kind_store,
            &supply_contract_store,
        );

        // Sixth group
        let order_service_store = self.gen_order_service(
            &order_store,
            &phone_store,
            &phone_model_store,
            &service_phone_model_store,
        );
        let warehouse_supply_store =
            self.gen_warehouse_supply(&warehouse_store, &supply_store, &supply_contract_store);

        // Seventh group
        let order_warehouse_store = self.gen_order_warehouse(
            &order_store,
            &order_service_store,
            &service_store,
            &component_kind_store,
            &phone_store,
            &phone_model_store,
            &component_store,
            &warehouse_store,
        );

        (
            component_kind_store,
            service_store,
            position_store,
            manufacturer_store,
            person_store,
            supplier_store,
            labor_contract_store,
            phone_model_store,
            staff_store,
            component_store,
            phone_store,
            account_store,
            supply_contract_store,
            order_store,
            supply_store,
            warehouse_store,
            service_phone_model_store,
            warehouse_supply_store,
            order_service_store,
            order_warehouse_store,
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            person_count: 250,
            supplier_count: 50,
            position_salary_scatter: 0.005,
            labor_contract_count: 25,
            labor_contract_weights: [2, 2, 14, 1, 1],
            staff_vacation_chance: 0.15,
            account_status_weights: [18, 1, 1],
            supply_contract_chance: 0.8,
            supply_contract_count: 3,
            phone_count: vec![10, 1],
            warehouse_variations: 5,
            warehouse_stock: (1, 3),
            warehouse_item_price_scatter: 0.5,
            order_count: 3,
            order_not_owner_chance: 0.01,
            supply_count: 5,
        }
    }
}

#[inline]
pub fn country_match(id: usize) -> &'static str {
    match id {
        0 => "US",
        1 => "CN",
        2 => "TW",
        _ => unreachable!(),
    }
}

pub fn rand_string(rng: &mut ThreadRng, length: usize) -> String {
    rng.sample_iter(Slice::new(&NUMBERS).unwrap())
        .take(length)
        .collect()
}

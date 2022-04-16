use std::ops::RangeInclusive;

use fake::{
    faker::{
        address::raw::*, company::raw::*, finance::raw::*, internet::raw::*, name::raw::*,
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
        Account, ComponentKind, LaborContract, Manufacturer, Person, Position, Service, Staff,
        Supplier,
    },
    types::{AccountStatus, ContractStatus, MetaTime, StaffStatus},
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
    Vec<Staff>,
    Vec<Account>,
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
            .collect::<Vec<_>>()
    }

    pub fn gen_manufacturer(&self) -> Vec<Manufacturer> {
        Manufacturer::SAMPLES
            .iter()
            .map(|(name, country)| Manufacturer::new_auto(name.to_string(), country.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn gen_component_kind(&self) -> Vec<ComponentKind> {
        ComponentKind::SAMPLES
            .iter()
            .map(|(name, desc)| {
                ComponentKind::new_auto(name.to_string(), desc.map(|val| val.to_string()))
            })
            .collect::<Vec<_>>()
    }

    pub fn gen_service(&self) -> Vec<Service> {
        Service::SAMPLES
            .iter()
            .map(|(name, desc)| {
                Service::new_auto(name.to_string(), desc.map(|val| val.to_string()))
            })
            .collect::<Vec<_>>()
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
    pub fn gen_labor_contract(&self, person_store: &[Person]) -> Vec<LaborContract> {
        if person_store.len() < self.labor_contract_count {
            panic!("Person count less than count of labor contracts");
        }

        let mut rng = thread_rng();
        let status_weights = WeightedIndex::new(self.labor_contract_weights).unwrap();
        let mut labor_contract_store = Vec::with_capacity(self.labor_contract_count);

        for _ in 0..self.labor_contract_count {
            loop {
                // FIX: Inf loop
                let p = person_store
                    .get(rng.gen_range(0..person_store.len()))
                    .unwrap();
                if !labor_contract_store
                    .iter()
                    .any(|c: &LaborContract| c.person == p.uuid)
                {
                    let status = ContractStatus::ALL[status_weights.sample(&mut rng)];
                    labor_contract_store.push(LaborContract::new_auto(
                        p.uuid,
                        rand_passport(&mut rng),
                        status,
                        match status {
                            ContractStatus::Review | ContractStatus::Negotiation => None,
                            ContractStatus::Active
                            | ContractStatus::Expired
                            | ContractStatus::Void => Some(MetaTime::now()),
                        },
                    ));
                    break;
                }
            }
        }

        labor_contract_store
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
            .collect::<Vec<_>>()
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

        // Second group
        let staff_store = self.gen_staff(&labor_contract_store, &position_store);

        //Third group
        let account_store = self.gen_account(&staff_store, &position_store);

        (
            component_kind_store,
            service_store,
            position_store,
            manufacturer_store,
            person_store,
            supplier_store,
            labor_contract_store,
            staff_store,
            account_store,
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            person_count: 250,
            supplier_count: 50,
            position_salary_scatter: 0.005,
            labor_contract_count: 15,
            labor_contract_weights: [2, 2, 14, 1, 1],
            staff_vacation_chance: 0.15,
            account_status_weights: [18, 1, 1],
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

pub fn rand_passport(rng: &mut ThreadRng) -> String {
    rng.sample_iter(Slice::new(&NUMBERS).unwrap())
        .take(10)
        .collect()
}

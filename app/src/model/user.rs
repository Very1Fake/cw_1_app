use cw_core::{
    tables::{Account, LaborContract, Person, Staff},
    types::{AccountRole, AccountStatus, ContractStatus, MetaTime, StaffStatus},
    uuid::Uuid,
};

#[derive(Clone, Debug)]
pub struct User {
    pub account: Account,
    pub staff: Staff,
    pub labor_contract: LaborContract,
    pub person: Person,
}

impl Default for User {
    fn default() -> Self {
        Self {
            account: Account {
                uuid: Uuid::new_v4(),
                staff: Uuid::nil(),
                login: String::from("very1fake"),
                password: String::from("password"),
                role: AccountRole::Admin,
                status: AccountStatus::Active,
                meta: MetaTime::default(),
            },
            staff: Staff {
                uuid: Uuid::new_v4(),
                contract: Uuid::nil(),
                position: Uuid::nil(),
                status: StaffStatus::Working,
            },
            labor_contract: LaborContract {
                uuid: Uuid::new_v4(),
                person: Uuid::nil(),
                passport: String::new(),
                status: ContractStatus::Active,
                signed: Some(MetaTime::now()),
                meta: MetaTime::default(),
            },
            person: Person {
                uuid: Uuid::new_v4(),
                first_name: String::from("Timur"),
                middle_name: None,
                last_name: String::from("Israpilov"),
                email: String::from("very1fake.coder@gmail.com"),
                phone: String::from("+79955927029"),
                meta: MetaTime::default(),
            },
        }
    }
}

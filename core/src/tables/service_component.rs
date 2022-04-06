use uuid::Uuid;

/// Represents relation table between [`Service`](`super::service::Service`) and [`ComponentKind`](`super::component_kind::ComponentKind`)
#[derive(Debug)]
pub struct ServiceComponent {
    /// Foreign key references [`Service`](`super::service::Service`)
    pub service: Uuid,
    /// Foreign key references [`ComponentKind`](`super::component_kind::ComponentKind`)
    pub component: Uuid,
}

impl ServiceComponent {
    pub const NAME: &'static str = "ServiceComponent";

    pub const CREATE: &'static str = r#"CREATE TABLE "ServiceComponent" (
    service uuid NOT NULL REFERENCES "Service" ON DELETE restrict ON UPDATE cascade,
    component uuid NOT NULL REFERENCES "ComponentKind" ON DELETE restrict ON UPDATE cascade,
    PRIMARY KEY (service, component)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "ServiceComponent";"#;

    pub const fn new(service: Uuid, component: Uuid) -> Self {
        Self { service, component }
    }
}

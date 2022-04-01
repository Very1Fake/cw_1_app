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
    pub const fn new(service: Uuid, component: Uuid) -> Self {
        Self { service, component }
    }
}

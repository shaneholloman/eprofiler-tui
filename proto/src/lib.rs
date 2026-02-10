#[allow(warnings)]
pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            pub mod v1 {
                include!("gen/opentelemetry.proto.common.v1.rs");
            }
        }
        pub mod resource {
            pub mod v1 {
                include!("gen/opentelemetry.proto.resource.v1.rs");
            }
        }
        pub mod profiles {
            pub mod v1development {
                include!("gen/opentelemetry.proto.profiles.v1development.rs");
            }
        }
        pub mod collector {
            pub mod profiles {
                pub mod v1development {
                    include!("gen/opentelemetry.proto.collector.profiles.v1development.rs");
                }
            }
        }
    }
}

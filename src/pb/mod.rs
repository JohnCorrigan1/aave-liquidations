// @generated
pub mod eth {
    pub mod erc721 {
        // @@protoc_insertion_point(attribute:eth.erc721.v1)
        pub mod v1 {
            include!("eth.erc721.v1.rs");
            // @@protoc_insertion_point(eth.erc721.v1)
        }
    }
}

// @@protoc_insertion_point(attribute:liquidations)
pub mod liquidations {
    include!("liquidations.rs");
    // @@protoc_insertion_point(liquidations)
}
pub mod sf {
    pub mod substreams {
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        pub mod sink {
            pub mod database {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.database.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.database.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.database.v1)
                }
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}

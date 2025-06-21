// This path needs to be relative to the directory *containing* this mod.rs file.
// If this file is in `src/`, and `gen_fake.rs` is in `src/gen/`,
// then the path is `gen/gen_fake.rs`.
#[path = "gen/gen_fake.rs"]
pub mod gen_fake_protobuf;

// Re-export FakeDataFieldOption directly from the generated module
pub use gen_fake_protobuf::FakeDataFieldOption;

// If you had other messages like `User` from `examples/user.proto`,
// you would add another module and re-export them, e.g.:
// #[path = "prost_generated/examples.rs"] // Assuming prost generates examples.rs
// pub mod examples_protobuf;
// pub use examples_protobuf::User;

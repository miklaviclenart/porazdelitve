pub mod utils;

pub mod porazdelitev;

pub use porazdelitev::Diskretna;
pub use porazdelitev::Porazdelitev;
pub use porazdelitev::PricakovanaVrednost;
pub use porazdelitev::Varianca;
pub use porazdelitev::Zvezna;

pub use porazdelitev::Bernoulli;
pub use porazdelitev::Binomska;
pub use porazdelitev::Eksponentna;
pub use porazdelitev::EnakomernaInterval;
pub use porazdelitev::Geometrijska;
pub use porazdelitev::Hipergeometrijska;
pub use porazdelitev::Normalna;
pub use porazdelitev::Poisson;

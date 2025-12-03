pub struct Subdivision(pub String);

impl From<&geoman::domain::enums::Country> for Subdivision {
    fn from(value: &geoman::domain::enums::Country) -> Self {
        let subdivison = match value {
            geoman::domain::enums::Country::SCOTLAND => "SCT",
            geoman::domain::enums::Country::ENGLAND => "ENG",
            geoman::domain::enums::Country::WALES => "WLS",
        };
        Self(subdivison.to_string())
    }
}

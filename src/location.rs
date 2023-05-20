#[derive(Debug)]
pub enum Country {
    UnitedStates, Canada, UnitedKingdom, Germany, France, Japan, Australia, China, Brazil, SouthKorea, Ireland, Spain, India, Switzerland
}

#[derive(Debug)]
pub enum Continent {
    NorthAmerica, Europe, Asia, Oceania, SouthAmerica
}

impl Country {
    pub fn country_to_continent(&self) -> Continent {
        match self {
            Country::UnitedStates | Country::Canada => Continent:: NorthAmerica,
            Country::UnitedKingdom | Country::Germany | Country::France | Country::Ireland | Country::Spain | Country::Switzerland => Continent::Europe,
            Country::Japan | Country::China | Country::SouthKorea | Country::India => Continent:: Asia,
            Country::Australia => Continent::Oceania,
            Country::Brazil => Continent::SouthAmerica,
        }
    }
}

impl std::str::FromStr for Country {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "USA" => Ok(Country::UnitedStates),
            "Canada" => Ok(Country::Canada),
            "UK" => Ok(Country::UnitedKingdom),
            "Germany" => Ok(Country::Germany),
            "France" => Ok(Country::France),
            "Ireland" => Ok(Country::Ireland),
            "Spain" => Ok(Country::Spain),
            "Switzerland" => Ok(Country::Switzerland),
            "Japan" => Ok(Country::Japan),
            "China" => Ok(Country::China),
            "South Korea" => Ok(Country::SouthKorea),
            "Australia" => Ok(Country::Australia),
            "Brazil" => Ok(Country::Brazil),
            "India" => Ok(Country::India),
            _ => Err("Invalid country name"),
        }
    }
}

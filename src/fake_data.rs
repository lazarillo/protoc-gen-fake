use fake::{
    Fake, // Import specific locale instances directly from `fake::locales`
    faker::{
        address::en::*, administrative::en::*, automotive::en::*, barcode::en::*, boolean::en::*,
        chrono::en::*, company::en::*, creditcard::en::*, currency::en::*, filesystem::en::*,
        finance::en::*, impls::*, internet::raw::*, job::en::*, lorem::raw::*, name::raw::*,
        number::en::*, phone_number::en::*,
    },
    locales::{AR_SA, DE_DE, Data, EN, FR_FR, IT_IT, JA_JP, PT_BR, PT_PT, ZH_CN, ZH_TW},
};
use rand::{Rng, rngs::ThreadRng};
use std::fmt; // Import Display trait for formatting // Import Rng for random number generation

// Define your comprehensive enum (no changes needed here)
pub enum FakeData {
    // Faker-specific types (String variants)
    CityPrefix(String),
    CitySuffix(String),
    CityName(String),
    CountryName(String),
    CountryCode(String),
    StreetSuffix(String),
    StreetName(String),
    TimeZone(String),
    StateName(String),
    StateAbbr(String),
    SecondaryAddressType(String),
    SecondaryAddress(String),
    ZipCode(String),
    PostCode(String),
    BuildingNumber(String),
    Latitude(String),
    Longitude(String),
    Geohash(String),
    Isbn(String),
    Isbn10(String),
    Isbn13(String),
    CreditCardNumber(String),
    CompanySuffix(String),
    CompanyName(String),
    Buzzword(String),
    BuzzwordMiddle(String),
    BuzzwordTail(String),
    CatchPhrase(String),
    BsVerb(String),
    BsAdj(String),
    BsNoun(String),
    Bs(String),
    Profession(String),
    Industry(String),
    FreeEmailProvider(String),
    DomainSuffix(String),
    FreeEmail(String),
    SafeEmail(String),
    Username(String),
    Password(String),
    IPv4(String),
    IPv6(String),
    IP(String),
    MACAddress(String),
    UserAgent(String),
    Seniority(String),
    Field(String),
    Position(String),
    Word(String),
    Words(Vec<String>),
    Sentence(String),
    Sentences(Vec<String>),
    Paragraph(String),
    Paragraphs(Vec<String>),
    FirstName(String),
    LastName(String),
    Title(String),
    Suffix(String),
    Name(String),
    NameWithTitle(String),
    PhoneNumber(String),
    CellNumber(String),
    FilePath(String),
    FileName(String),
    FileExtension(String),
    DirPath(String),
    MimeType(String),
    Semver(String),
    SemverStable(String),
    SemverUnstable(String),
    CurrencyCode(String),
    CurrencyName(String),
    CurrencySymbol(String),
    Bic(String),
    Isin(String),
    HexColor(String),
    RgbColor(String),
    RgbaColor(String),
    HslColor(String),
    HslaColor(String),
    Color(String),
    Time(String),
    Date(String),
    DateTime(String),
    RfcStatusCode(String),
    ValidStatusCode(String),

    // Custom types, still using Faker to generate them
    Age(u32),
    Other(String), // For the default case
}

impl fmt::Display for FakeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FakeData::CityPrefix(s)
            | FakeData::CitySuffix(s)
            | FakeData::CityName(s)
            | FakeData::CountryName(s)
            | FakeData::CountryCode(s)
            | FakeData::StreetSuffix(s)
            | FakeData::StreetName(s)
            | FakeData::TimeZone(s)
            | FakeData::StateName(s)
            | FakeData::StateAbbr(s)
            | FakeData::SecondaryAddressType(s)
            | FakeData::SecondaryAddress(s)
            | FakeData::ZipCode(s)
            | FakeData::PostCode(s)
            | FakeData::BuildingNumber(s)
            | FakeData::Latitude(s)
            | FakeData::Longitude(s)
            | FakeData::Geohash(s)
            | FakeData::Isbn(s)
            | FakeData::Isbn10(s)
            | FakeData::Isbn13(s)
            | FakeData::CreditCardNumber(s)
            | FakeData::CompanySuffix(s)
            | FakeData::CompanyName(s)
            | FakeData::Buzzword(s)
            | FakeData::BuzzwordMiddle(s)
            | FakeData::BuzzwordTail(s)
            | FakeData::CatchPhrase(s)
            | FakeData::BsVerb(s)
            | FakeData::BsAdj(s)
            | FakeData::BsNoun(s)
            | FakeData::Bs(s)
            | FakeData::Profession(s)
            | FakeData::Industry(s)
            | FakeData::FreeEmailProvider(s)
            | FakeData::DomainSuffix(s)
            | FakeData::FreeEmail(s)
            | FakeData::SafeEmail(s)
            | FakeData::Username(s)
            | FakeData::Password(s)
            | FakeData::IPv4(s)
            | FakeData::IPv6(s)
            | FakeData::IP(s)
            | FakeData::MACAddress(s)
            | FakeData::UserAgent(s)
            | FakeData::Seniority(s)
            | FakeData::Field(s)
            | FakeData::Position(s)
            | FakeData::Word(s)
            | FakeData::Sentence(s)
            | FakeData::FirstName(s)
            | FakeData::LastName(s)
            | FakeData::Title(s)
            | FakeData::Suffix(s)
            | FakeData::Name(s)
            | FakeData::NameWithTitle(s)
            | FakeData::PhoneNumber(s)
            | FakeData::CellNumber(s)
            | FakeData::FilePath(s)
            | FakeData::FileName(s)
            | FakeData::FileExtension(s)
            | FakeData::DirPath(s)
            | FakeData::MimeType(s)
            | FakeData::Semver(s)
            | FakeData::SemverStable(s)
            | FakeData::SemverUnstable(s)
            | FakeData::CurrencyCode(s)
            | FakeData::CurrencyName(s)
            | FakeData::CurrencySymbol(s)
            | FakeData::Bic(s)
            | FakeData::Isin(s)
            | FakeData::HexColor(s)
            | FakeData::RgbColor(s)
            | FakeData::RgbaColor(s)
            | FakeData::HslColor(s)
            | FakeData::HslaColor(s)
            | FakeData::Color(s)
            | FakeData::Time(s)
            | FakeData::Date(s)
            | FakeData::DateTime(s)
            | FakeData::RfcStatusCode(s)
            | FakeData::ValidStatusCode(s)
            | FakeData::Paragraph(s)
            | FakeData::Other(s) => write!(f, "{}", s),
            FakeData::Age(i) => write!(f, "{}", i),
            FakeData::Words(v) | FakeData::Sentences(v) | FakeData::Paragraphs(v) => {
                write!(f, "{:?}", v)
            }
        }
    }
}

macro_rules! generate_faker_match_arms {
    // Main entry point for the macro
    // This macro generates match arms for different fakers based on the provided data type and language
    (
        $data_type_var:ident,
        $language_var:ident,
        // List 1: All fakers that are functions and take NO arguments (e.g., FirstName(), City())
        [ $( ($string_key_no_arg:literal, $faker_path_no_arg:path, $enum_variant_no_arg:ident) ),* ],
        // List 2: All fakers that are functions and DO take arguments (e.g., Words(range), Password(len))
        [ $( ($string_key_arg:literal, $faker_path_arg:path, $enum_variant_arg:ident, $arg_expr:expr) ),* ]
    ) => {
        // Outer match on language
        match $language_var.to_lowercase().as_str() {
            "ar_sa" | "ar" | "english" => {
                // Call the internal rule, passing the concrete locale instance
                generate_faker_match_arms!(@internal_faker_match $data_type_var, AR_SA,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "de_de" | "de" | "german" => {
                // Call the internal rule, passing the concrete locale instance
                generate_faker_match_arms!(@internal_faker_match $data_type_var, DE_DE,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "en" | "english" => {
                // Call the internal rule, passing the concrete locale instance
                generate_faker_match_arms!(@internal_faker_match $data_type_var, EN,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "fr_fr" | "fr" | "french" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, FR_FR,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "it_it" | "it" | "italian" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, IT_IT,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "ja_jp" | "ja" | "japanese" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, JA_JP,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "pt_br" | "pt" | "brazilian portuguese" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, PT_BR,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "pt_pt"  | "portuguese" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, PT_PT,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "zh_cn" | "zh" | "chinese" | "simplified chinese" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, ZH_CN,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            "zh_tw" | "taiwanese" | "traditional chinese" => {
                generate_faker_match_arms!(@internal_faker_match $data_type_var, ZH_TW,
                    [ $( ($string_key_no_arg, $faker_path_no_arg, $enum_variant_no_arg) ),* ],
                    [ $( ($string_key_arg, $faker_path_arg, $enum_variant_arg, $arg_expr) ),* ]
                )
            },
            // Add more language arms here for each locale you support
            _ => None, // Fallback if the language_input itself is not recognized
        }
    };
    // Internal match on the data type
    // It is called by the outer rule, receiving the data_type_var, the concrete locale_instance,
    // and the lists of fakers.
    (@internal_faker_match
        $data_type_var:ident,
        $locale_instance:expr, // This is now the concrete locale (e.g., EN_US) passed from above
        [ $( ($string_key_no_arg:literal, $faker_path_no_arg:path, $enum_variant_no_arg:ident) ),* ],
        [ $( ($string_key_arg:literal, $faker_path_arg:path, $enum_variant_arg:ident, $arg_expr:expr) ),* ]
    ) => {
        match $data_type_var {
            // Match arms for no-argument fakers: Call the function (e.g., `FirstName()`), then `.fake_with_rng()`
            $(
                $string_key_no_arg => Some(
                    FakeData::$enum_variant_no_arg(
                    ($faker_path_no_arg)($locale_instance).fake_with_rng(&mut ThreadRng::default())
                    )
                ),
            )*
            // Match arms for fakers with constructor arguments: Call the function with args (e.g., `Words(0..10)`), then `.fake_with_rng()`
            $(
                $string_key_arg => Some(
                    FakeData::$enum_variant_arg(
                        ($faker_path_arg)($locale_instance, $arg_expr).fake_with_rng(&mut ThreadRng::default())
                    )
                ),
            )*
            _ => None, // Fallback if no specific faker matches for this data_type
        }
    };
}

pub fn get_fake_data(data_type: &str, language: &str) -> Option<FakeData> {
    let result = generate_faker_match_arms!(
        data_type, // <<< FIX: Passing `data_type` as the first argument
        language,  // This is now the second argument (the locale variable)
        // --- START OF FIRST LIST (Unit Struct Fakers - call method directly) ---
        [
            ("FirstName", FirstName, FirstName),
            ("LastName", LastName, LastName),
            ("SafeEmail", SafeEmail, SafeEmail)
        ],
        // --- START OF SECOND LIST (Fakers with Range<usize> constructor arguments) ---
        [
            ("Words", Words, Words, 0..10),
            ("Sentences", Sentences, Sentences, 0..10),
            ("Paragraphs", Paragraphs, Paragraphs, 0..10)
        ] // --- END OF SECOND LIST ---
    );

    if let Some(data) = result {
        return Some(data);
    }

    // Special handling for "Age" (not a fake crate faker)
    if data_type == "Age" {
        return Some(FakeData::Age(rand::rng().random_range(8..90)));
    }
    None
}

// pub fn get_fake_data<L>(data_type: &str, locale: L) -> FakeData
// where
//     L: fake::locales::Data + Copy + 'static,
// {
//     let result = generate_faker_match_arms!(
//         data_type, // <<< FIX: Passing `data_type` as the first argument
//         locale,    // This is now the second argument (the locale variable)
//         // --- START OF FIRST LIST (Unit Struct Fakers - call method directly) ---
//         [
//             ("FirstName", FirstName, FirstName),
//             ("LastName", LastName, LastName),
//             ("Suffix", Suffix, Suffix),
//             ("Name", Name, Name),
//             ("NameWithTitle", NameWithTitle, NameWithTitle),
//             ("Seniority", Seniority, Seniority),
//             ("Field", Field, Field),
//             ("Position", Position, Position),
//             ("Semver", Semver, Semver),
//             ("SemverStable", SemverStable, SemverStable),
//             ("SemverUnstable", SemverUnstable, SemverUnstable),
//             ("Isbn", Isbn, Isbn),
//             ("Isbn10", Isbn10, Isbn13),
//             ("Isbn13", Isbn13, Isbn13),
//             ("CreditCardNumber", CreditCardNumber, CreditCardNumber),
//             ("CompanySuffix", CompanySuffix, CompanySuffix),
//             ("CompanyName", CompanyName, CompanyName),
//             ("Buzzword", Buzzword, Buzzword),
//             ("BuzzwordMiddle", BuzzwordMiddle, BuzzwordMiddle),
//             ("BuzzwordTail", BuzzwordTail, BuzzwordTail),
//             ("CatchPhrase", CatchPhrase, CatchPhrase),
//             ("BsVerb", BsVerb, BsVerb),
//             ("BsAdj", BsAdj, BsAdj),
//             ("BsNoun", BsNoun, BsNoun),
//             ("Bs", Bs, Bs),
//             ("Profession", Profession, Profession),
//             ("Industry", Industry, Industry),
//             ("FreeEmailProvider", FreeEmailProvider, FreeEmailProvider),
//             ("DomainSuffix", DomainSuffix, DomainSuffix),
//             ("FreeEmail", FreeEmail, FreeEmail),
//             ("SafeEmail", SafeEmail, SafeEmail),
//             ("Username", Username, Username),
//             ("Password", Password, Password),
//             ("IPv4", IPv4, IPv4),
//             ("IPv6", IPv6, IPv6),
//             ("IP", IP, IP),
//             ("MACAddress", MACAddress, MACAddress),
//             ("UserAgent", UserAgent, UserAgent),
//             ("FilePath", FilePath, FilePath),
//             ("FileName", FileName, FileName),
//             ("FileExtension", FileExtension, FileExtension),
//             ("DirPath", DirPath, DirPath),
//             ("MimeType", MimeType, MimeType),
//             ("CurrencyCode", CurrencyCode, CurrencyCode),
//             ("CurrencyName", CurrencyName, CurrencyName),
//             ("CurrencySymbol", CurrencySymbol, CurrencySymbol),
//             ("Bic", Bic, Bic),
//             ("Isin", Isin, Isin),
//             ("Time", Time, Time),
//             ("Date", Date, Date),
//             ("DateTime", DateTime, DateTime),
//             ("PhoneNumber", PhoneNumber, PhoneNumber),
//             ("CellNumber", CellNumber, CellNumber),
//             // <<< FIX: Changed CityName to City, and CountryName to Country
//             ("CityPrefix", CityPrefix, CityPrefix),
//             ("CitySuffix", CitySuffix, CitySuffix),
//             ("CountryCode", CountryCode, CountryCode),
//             ("StreetSuffix", StreetSuffix, StreetSuffix),
//             ("TimeZone", TimeZone, TimeZone),
//             ("StateName", StateName, StateName),
//             ("StateAbbr", StateAbbr, StateAbbr),
//             (
//                 "SecondaryAddressType",
//                 SecondaryAddressType,
//                 SecondaryAddressType
//             ),
//             ("SecondaryAddress", SecondaryAddress, SecondaryAddress),
//             ("ZipCode", ZipCode, ZipCode),
//             ("PostCode", PostCode, PostCode),
//             ("BuildingNumber", BuildingNumber, BuildingNumber),
//             ("Latitude", Latitude, Latitude),
//             ("Longitude", Longitude, Longitude),
//             ("Geohash", Geohash, Geohash),
//             ("Word", Word, Word),
//             ("Sentence", Sentence, Sentence),
//             ("Paragraph", Paragraph, Paragraph)
//         ], // --- END OF FIRST LIST ---
//         [("StreetName", StreetName, StreetName)], // <--- This comma is essential to separate it from the next list
//         // --- START OF SECOND LIST (Fakers with Range<usize> constructor arguments) ---
//         [
//             ("Words", Words, Words, 0..10),
//             ("Sentences", Sentences, Sentences, 0..10),
//             ("Paragraphs", Paragraphs, Paragraphs, 0..10)
//         ] // --- END OF SECOND LIST ---
//     );

//     if let Some(data) = result {
//         return data;
//     }

//     // Special handling for "Age" (not a fake crate faker)
//     if data_type == "Age" {
//         return FakeData::Age(rand::rng().random_range(8..90));
//     }

//     // Fallback for unknown data types
//     FakeData::Other("Nothing".to_string())
// }

// pub enum LanguageLookup {
//     Arabic(AR_SA),
//     ChineseSimplified(ZH_CN),
//     ChineseTraditional(ZH_TW),
//     English(EN),
//     French(FR_FR),
//     German(DE_DE),
//     Italian(IT_IT),
//     Japanese(JA_JP),
//     PortugueseBrazil(PT_BR),
//     PortuguesePortugal(PT_PT),
// }

// fn get_lang(language: &str) -> Option<&'static dyn Locale> {
//     // This function allows for a more flexible way to provide a language code
//     let lower_lang = language.to_lowercase().as_str();
//     match lower_lang {
//         "en" => EN,
//         "fr" => FR_FR,
//         "pt_br" => PT_BR,
//         _ => EN, // Default to English if the language is not recognized
//     }
// }

fn main() {
    // SafeEmail().fake_with_rng(&mut ThreadRng::default());

    // Example usage with EN locale
    let name_en: Option<FakeData> = get_fake_data("FirstName", "English");
    match name_en {
        Some(s) => println!("Generated Name (EN): {}", s),
        _ => unreachable!(),
    }

    let city_en = get_fake_data("CityName", "English");
    match city_en {
        Some(s) => println!("Generated City (EN): {}", s),
        _ => unreachable!(),
    }

    // Example usage with French locale
    let name_fr = get_fake_data("Name", "French");
    match name_fr {
        Some(s) => println!("Generated Name (FR_FR): {}", s),
        _ => unreachable!(),
    }

    // let city_fr = get_fake_data("CityName", FR_FR);
    // match city_fr {
    //     // Note: For FR, 'City' might give a French city name, even if imported from en::*
    //     // This is the beauty of fake_with_context.
    //     FakeData::CityName(s) => println!("Generated City (FR_FR): {}", s),
    //     _ => unreachable!(),
    // }

    // let words_fr = get_fake_data("Words", FR_FR);
    // match words_fr {
    //     FakeData::Words(s) => println!("Generated Words (FR_FR): {:?}", s),
    //     _ => unreachable!(),
    // }

    // // Example usage with Brazilian Portuguese locale
    // let name_pt_br = get_fake_data("Name", PT_BR);
    // match name_pt_br {
    //     FakeData::Name(s) => println!("Generated Name (PT_BR): {}", s),
    //     _ => unreachable!(),
    // }

    // let unknown = get_fake_data("NonExistentType", EN);
    // match unknown {
    //     FakeData::Other(s) => println!("Generated Other: {}", s),
    //     _ => unreachable!(),
    // }
}

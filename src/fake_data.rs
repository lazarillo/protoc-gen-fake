use fake::{
    Fake, // Import specific locale instances directly from `fake::locales`
    faker::{
        address::en::*, administrative::en::*, automotive::en::*, barcode::en::*, boolean::en::*,
        chrono::en::*, company::en::*, creditcard::en::*, currency::en::*, filesystem::en::*,
        finance::en::*, impls::*, internet::en::*, job::en::*, lorem::en::*, name::en::*,
        number::en::*, phone_number::en::*,
    },
    locales::{Data, EN, FR_FR, PT_BR},
};
use rand::{Rng, rngs::ThreadRng}; // Import Rng for random number generation

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

// Macro definition: 'echo_params!'
// It takes three arguments: a literal string, an identifier, and an expression.
// All arguments are separated by commas. The outer delimiters are parentheses ().
macro_rules! echo_params {
    ($message:literal, $tag:ident, $value:expr) => {
        println!(
            "ECHO: Message: {}, Tag: {}, Value: {}",
            $message,
            stringify!($tag),
            $value
        );
    };
}

// Function that calls the 'echo_params!' macro
fn demonstrate_echo_params() {
    println!("\n--- Demonstrating echo_params! ---");
    echo_params!("Hello World", item_name, 123);
    echo_params!("Another string", some_id, 45.67);
    echo_params!("Final message", status, true);
    println!("--- End echo_params! Demo ---");
}

macro_rules! generate_faker_match_arms {
    (
        $data_type_var:ident,
        $locale_var:ident,
        // List 1: All fakers that are functions and take NO arguments (e.g., FirstName(), City())
        [ $( ($string_key_no_arg:literal, $faker_path_no_arg:path, $enum_variant_no_arg:ident) ),* ],
        // List 2: All fakers that are functions and DO take arguments (e.g., Words(range), Password(len))
        [ $( ($string_key_arg:literal, $faker_path_arg:path, $enum_variant_arg:ident, $arg_expr:expr) ),* ]
    ) => {
        match $data_type_var {
            // Match arms for no-argument fakers: Call the function (e.g., `FirstName()`), then `.fake_with_rng()`
            $(
                $string_key_no_arg => Some(
                    FakeData::$enum_variant_no_arg(
                    ($faker_path_no_arg)().fake_with_rng(&mut ThreadRng::default())
                    )
                ),
            )*
            // Match arms for fakers with constructor arguments: Call the function with args (e.g., `Words(0..10)`), then `.fake_with_rng()`
            $(
                $string_key_arg => Some(
                    FakeData::$enum_variant_arg(
                        ($faker_path_arg)($arg_expr).fake_with_rng(&mut ThreadRng::default())
                    )
                ),
            )*
            _ => None, // Fallback if no specific faker matches
        }
    };
}

pub fn testing1() -> String {
    FirstName().fake()
    // FirstName().fake_with_rng(&mut ThreadRng::default())
}

pub fn testing2() -> String {
    LastName().fake()
    // FirstName().fake_with_rng(&mut ThreadRng::default())
}

pub fn get_fake_data<L>(data_type: &str, locale: L) -> FakeData
where
    L: fake::locales::Data + Copy + 'static,
{
    let result = generate_faker_match_arms!(
        data_type, // <<< FIX: Passing `data_type` as the first argument
        locale,    // This is now the second argument (the locale variable)
        // --- START OF FIRST LIST (Unit Struct Fakers - call method directly) ---
        [
            ("FirstName", FirstName, FirstName),
            ("LastName", LastName, LastName)
        ],
        // --- START OF SECOND LIST (Fakers with Range<usize> constructor arguments) ---
        [
            ("Words", Words, Words, 0..10),
            ("Sentences", Sentences, Sentences, 0..10),
            ("Paragraphs", Paragraphs, Paragraphs, 0..10)
        ] // --- END OF SECOND LIST ---
    );

    if let Some(data) = result {
        return data;
    }

    // Special handling for "Age" (not a fake crate faker)
    if data_type == "Age" {
        return FakeData::Age(rand::rng().random_range(8..90));
    }

    // Fallback for unknown data types
    FakeData::Other("Nothing".to_string())
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

fn main() {
    // Example usage with EN locale
    let name_en = get_fake_data("Name", EN);
    match name_en {
        FakeData::Name(s) => println!("Generated Name (EN): {}", s),
        _ => unreachable!(),
    }

    let city_en = get_fake_data("CityName", EN);
    match city_en {
        FakeData::CityName(s) => println!("Generated City (EN): {}", s),
        _ => unreachable!(),
    }

    let words_en: FakeData = get_fake_data("Words", EN);
    match words_en {
        FakeData::Words(s) => println!("Generated Words (EN): {:?}", s),
        _ => unreachable!(),
    }

    let age = get_fake_data("Age", EN);
    match age {
        FakeData::Age(a) => println!("Generated Age: {}", a),
        _ => unreachable!(),
    }

    // Example usage with French locale
    let name_fr = get_fake_data("Name", FR_FR);
    match name_fr {
        FakeData::Name(s) => println!("Generated Name (FR_FR): {}", s),
        _ => unreachable!(),
    }

    let city_fr = get_fake_data("CityName", FR_FR);
    match city_fr {
        // Note: For FR, 'City' might give a French city name, even if imported from en::*
        // This is the beauty of fake_with_context.
        FakeData::CityName(s) => println!("Generated City (FR_FR): {}", s),
        _ => unreachable!(),
    }

    let words_fr = get_fake_data("Words", FR_FR);
    match words_fr {
        FakeData::Words(s) => println!("Generated Words (FR_FR): {:?}", s),
        _ => unreachable!(),
    }

    // Example usage with Brazilian Portuguese locale
    let name_pt_br = get_fake_data("Name", PT_BR);
    match name_pt_br {
        FakeData::Name(s) => println!("Generated Name (PT_BR): {}", s),
        _ => unreachable!(),
    }

    let unknown = get_fake_data("NonExistentType", EN);
    match unknown {
        FakeData::Other(s) => println!("Generated Other: {}", s),
        _ => unreachable!(),
    }
}

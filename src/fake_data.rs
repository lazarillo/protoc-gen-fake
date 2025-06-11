// use fake::faker::address::raw::*;
// use fake::faker::administrative::raw::*;
// use fake::locales::*;
// use fake::{Fake, Faker};

// fn generate_city() -> String {
//     BuildingNumber(EN).fake::<String>()
//     // Faker.fake::<(u8, u32, u32)>()
// }

// // Define an enum to encapsulate the possible return types
// pub enum FakeData {
//     // Faker-specific types
//     CityPrefix(String),
//     CitySuffix(String),
//     CityName(String),
//     CountryName(String),
//     CountryCode(String),
//     StreetSuffix(String),
//     StreetName(String),
//     TimeZone(String),
//     StateName(String),
//     StateAbbr(String),
//     SecondaryAddressType(String),
//     SecondaryAddress(String),
//     ZipCode(String),
//     PostCode(String),
//     BuildingNumber(String),
//     Latitude(String),
//     Longitude(String),
//     Geohash(String),
//     Isbn(String),
//     Isbn10(String),
//     Isbn13(String),
//     CreditCardNumber(String),
//     CompanySuffix(String),
//     CompanyName(String),
//     Buzzword(String),
//     BuzzwordMiddle(String),
//     BuzzwordTail(String),
//     CatchPhrase(String),
//     BsVerb(String),
//     BsAdj(String),
//     BsNoun(String),
//     Bs(String),
//     Profession(String),
//     Industry(String),
//     FreeEmailProvider(String),
//     DomainSuffix(String),
//     FreeEmail(String),
//     SafeEmail(String),
//     Username(String),
//     Password(String),
//     IPv4(String),
//     IPv6(String),
//     IP(String),
//     MACAddress(String),
//     UserAgent(String),
//     Seniority(String),
//     Field(String),
//     Position(String),
//     Word(String),
//     Words(String),
//     Sentence(String),
//     Sentences(String),
//     Paragraph(String),
//     Paragraphs(String),
//     FirstName(String),
//     LastName(String),
//     Title(String),
//     Suffix(String),
//     Name(String),
//     NameWithTitle(String),
//     PhoneNumber(String),
//     CellNumber(String),
//     FilePath(String),
//     FileName(String),
//     FileExtension(String),
//     DirPath(String),
//     MimeType(String),
//     Semver(String),
//     SemverStable(String),
//     SemverUnstable(String),
//     CurrencyCode(String),
//     CurrencyName(String),
//     CurrencySymbol(String),
//     Bic(String),
//     Isin(String),
//     HexColor(String),
//     RgbColor(String),
//     RgbaColor(String),
//     HslColor(String),
//     HslaColor(String),
//     Color(String),
//     Time(String),
//     Date(String),
//     DateTime(String),
//     RfcStatusCode(String),
//     ValidStatusCode(String),

//     // Custom types, still using Faker to generate them
//     Age(u32),
//     Other(String), // For the default case
// }

// fn generate_tuple() -> (u8, u32, u32) {
//     Faker.fake::<(u8, u32, u32)>()
// }

// fn gen_fake_data(data_type: &str, locale: &str) -> String {
//     match data_type {
//         "CityName" => CityName(locale).fake::<String>(),
//         "CountryName" => CountryName(locale).fake::<String>(),
//         "StateName" => StateName(locale).fake::<String>(),
//         "StreetName" => StreetName(locale).fake::<String>(),
//         "ZipCode" => ZipCode(locale).fake::<String>(),
//         _ => format!("<unsupported fake data type: {}>", data_type),
//     }
// }

// // Macro to generate match arms for faker types returning String
// macro_rules! generate_faker_match_arms {
//     (
//         $( ($string_key:literal, $faker_type:path, $enum_variant:ident) ),*
//         $(,)? // Allow trailing comma
//     ) => {
//         // The function now takes a generic locale parameter `L` which must implement `Locale`
//         fn gen_fake_data<L>(data_type: &str, locale: L) -> FakeData
//         where
//             L: Locale + Copy + 'static, // Common bounds for Locale instances
//         {
//             match data_type {
//                 $(
//                     // Use .fake_with_locale() and pass the 'locale' argument
//                     $string_key => FakeData::$enum_variant($faker_type.fake_with_locale(locale)),
//                 )*
//                 // Custom handling for Age (no locale needed as it's not a faker call)
//                 "Age" => FakeData::Age(rand::thread_rng().gen_range(8..90)),
//                 // Default case (no locale needed)
//                 _ => FakeData::Other("Nothing".to_string()),
//             }
//         }
//     };
// }

// // const f: &str = CityName().fake::<String>();

// //   CityPrefix
// //   CitySuffix
// //   CityName
// //   CountryName
// //   CountryCode
// //   StreetSuffix
// //   StreetName
// //   TimeZone
// //   StateName
// //   StateAbbr
// //   SecondaryAddressType
// //   SecondaryAddress
// //   ZipCode
// //   PostCode
// //   BuildingNumber
// //   Latitude
// //   Longitude
// //   Geohash
// //   Isbn
// //   Isbn10
// //   Isbn13
// //   CreditCardNumber
// //   CompanySuffix
// //   CompanyName
// //   Buzzword
// //   BuzzwordMiddle
// //   BuzzwordTail
// //   CatchPhrase
// //   BsVerb
// //   BsAdj
// //   BsNoun
// //   Bs
// //   Profession
// //   Industry
// //   FreeEmailProvider
// //   DomainSuffix
// //   FreeEmail
// //   SafeEmail
// //   Username
// //   Password
// //   IPv4
// //   IPv6
// //   IP
// //   MACAddress
// //   UserAgent
// //   Seniority
// //   Field
// //   Position
// //   Word
// //   Words
// //   Sentence
// //   Sentences
// //   Paragraph
// //   Paragraphs
// //   FirstName
// //   LastName
// //   Title
// //   Suffix
// //   Name
// //   NameWithTitle
// //   PhoneNumber
// //   CellNumber
// //   FilePath

// // FileName
// //   FileExtension
// //   DirPath
// //   MimeType
// //   Semver
// //   SemverStable
// //   SemverUnstable
// //   CurrencyCode
// //   CurrencyName
// //   CurrencySymbol
// //   Bic
// //   Isin
// //   HexColor
// //   RgbColor
// //   RgbaColor
// //   HslColor
// //   HslaColor
// //   Color
// //   Time
// //   Date
// //   DateTime
// //   RfcStatusCode
// //   ValidStatusCode

// use fake::faker::address::raw::*;
// use fake::faker::administrative::raw::*;
// use fake::locales::*;
// use fake::{Fake, Faker};

use fake::{
    Fake,
    // Import `raw` fakers from their respective modules
    faker::address::raw::*,
    // IMPORTANT: Corrected Imports for ALL relevant GenFn traits
    // These traits are defined *within* their respective `raw` faker modules.
    faker::address::{
        BuildingNumberGenFn, CityNameGenFn, CityPrefixGenFn, CitySuffixGenFn, CountryCodeGenFn,
        CountryNameGenFn, GeohashGenFn, LatitudeGenFn, LongitudeGenFn, PostCodeGenFn,
        SecondaryAddressGenFn, SecondaryAddressTypeGenFn, StateAbbrGenFn, StateNameGenFn,
        StreetNameGenFn, StreetSuffixGenFn, TimeZoneGenFn, ZipCodeGenFn,
    },
    faker::auto::raw::*,
    faker::auto::raw::{
        CreditCardNumberGenFn, Isbn10GenFn, Isbn13GenFn, IsbnGenFn, SemverGenFn, SemverStableGenFn,
        SemverUnstableGenFn,
    },
    faker::color::raw::*,
    faker::color::raw::{
        ColorGenFn, HexColorGenFn, HslColorGenFn, HslaColorGenFn, RgbColorGenFn, RgbaColorGenFn,
    },
    faker::company::raw::*,
    faker::company::raw::{
        BsAdjGenFn, BsGenFn, BsNounGenFn, BsVerbGenFn, BuzzwordGenFn, BuzzwordMiddleGenFn,
        BuzzwordTailGenFn, CatchPhraseGenFn, CompanyNameGenFn, CompanySuffixGenFn, IndustryGenFn,
        ProfessionGenFn,
    },
    faker::filesystem::raw::*,
    faker::filesystem::raw::{
        DirPathGenFn, FileExtensionGenFn, FileNameGenFn, FilePathGenFn, MimeTypeGenFn,
    },
    faker::finance::raw::*,
    faker::finance::raw::{
        BicGenFn, CurrencyCodeGenFn, CurrencyNameGenFn, CurrencySymbolGenFn, IsinGenFn,
    },
    faker::http::raw::*,

    faker::http::raw::{RfcStatusCodeGenFn, ValidStatusCodeGenFn},
    faker::internet::raw::*,
    faker::internet::raw::{
        DomainSuffixGenFn, FreeEmailGenFn, FreeEmailProviderGenFn, IpGenFn, Ipv4GenFn, Ipv6GenFn,
        MacAddressGenFn, PasswordGenFn, SafeEmailGenFn, UserAgentGenFn, UsernameGenFn,
    },
    faker::lorem::raw::*,
    faker::lorem::raw::{
        ParagraphGenFn, ParagraphsGenFn, SentenceGenFn, SentencesGenFn, WordGenFn, WordsGenFn,
    },
    faker::name::raw::*,
    faker::name::raw::{
        FieldGenFn, FirstNameGenFn, LastNameGenFn, NameGenFn, NameWithTitleGenFn, PositionGenFn,
        SeniorityGenFn, SuffixGenFn, TitleGenFn,
    },
    faker::phone_number::raw::*,
    faker::phone_number::raw::{CellNumberGenFn, PhoneNumberGenFn},
    faker::time::raw::*,
    faker::time::raw::{DateGenFn, DateTimeGenFn, TimeGenFn},
    locales::Data,            // The correct trait for locale types in fake v4.x
    locales::{EN, FR, PT_BR}, // Import specific locale instances directly from `fake::locales`
};

fn generate_city() -> String {
    CityName(EN).fake::<String>()
    // Faker.fake::<(u8, u32, u32)>()
}

// Explicitly import rand for thread_rng and gen_range
use rand::Rng; // For the age range generation

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
    Words(String),
    Sentence(String),
    Sentences(String),
    Paragraph(String),
    Paragraphs(String),
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

macro_rules! generate_one_arg_fakers {
    (
        $( ($string_key:literal, $faker_type_path:path, $enum_variant:ident) ),*
        $(,)?
    ) => {
        // This function handles fakers that take only `locale` as an argument.
        fn get_one_arg_fake_data<L>(data_type: &str, locale: L) -> Option<FakeData>
        where
            L: Data + Copy + 'static // Standard locale bounds
               // All GenFn traits for fakers in this category:
               + FirstNameGenFn + LastNameGenFn + TitleGenFn + SuffixGenFn + NameGenFn + NameWithTitleGenFn
               + SeniorityGenFn + FieldGenFn + PositionGenFn
               + SemverGenFn + SemverStableGenFn + SemverUnstableGenFn
               + IsbnGenFn + Isbn10GenFn + Isbn13GenFn + CreditCardNumberGenFn
               + CompanySuffixGenFn + CompanyNameGenFn + BuzzwordGenFn + BuzzwordMiddleGenFn
               + BuzzwordTailGenFn + CatchPhraseGenFn + BsVerbGenFn + BsAdjGenFn + BsNounGenFn + BsGenFn
               + ProfessionGenFn + IndustryGenFn
               + FreeEmailProviderGenFn + DomainSuffixGenFn + FreeEmailGenFn + SafeEmailGenFn
               + UsernameGenFn + PasswordGenFn + Ipv4GenFn + Ipv6GenFn + IpGenFn + MacAddressGenFn
               + UserAgentGenFn
               + FilePathGenFn + FileNameGenFn + FileExtensionGenFn + DirPathGenFn + MimeTypeGenFn
               + CurrencyCodeGenFn + CurrencyNameGenFn + CurrencySymbolGenFn + BicGenFn + IsinGenFn
               + HexColorGenFn + RgbColorGenFn + RgbaColorGenFn + HslColorGenFn + HslaColorGenFn + ColorGenFn
               + TimeGenFn + DateGenFn + DateTimeGenFn
               + RfcStatusCodeGenFn + ValidStatusCodeGenFn
               + PhoneNumberGenFn + CellNumberGenFn,
        {
            match data_type {
                $(
                    $string_key => Some(FakeData::$enum_variant( ($faker_type_path)(locale).fake::<String>() )),
                )*
                _ => None, // If data_type doesn't match any in this category
            }
        }
    };
}

macro_rules! generate_two_arg_u8_fakers {
    (
        $( ($string_key:literal, $faker_type_path:path, $enum_variant:ident) ),*
        $(,)?
    ) => {
        // This function handles fakers that take `locale` and a `u8` argument.
        fn get_two_arg_u8_fake_data<L>(data_type: &str, locale: L) -> Option<FakeData>
        where
            L: Data + Copy + 'static // Standard locale bounds
               // All GenFn traits for fakers in this category:
               + CityPrefixGenFn + CitySuffixGenFn + CityNameGenFn + CountryNameGenFn + CountryCodeGenFn
               + StreetSuffixGenFn + StreetNameGenFn + TimeZoneGenFn + StateNameGenFn + StateAbbrGenFn
               + SecondaryAddressTypeGenFn + SecondaryAddressGenFn + ZipCodeGenFn + PostCodeGenFn
               + BuildingNumberGenFn + LatitudeGenFn + LongitudeGenFn + GeohashGenFn,
        {
            match data_type {
                $(
                    $string_key => Some(FakeData::$enum_variant( ($faker_type_path)(locale, 0u8).fake::<String>() )),
                )*
                _ => None, // If data_type doesn't match any in this category
            }
        }
    };
}

macro_rules! generate_range_arg_fakers {
    (
        $( ($string_key:literal, $faker_type_path:path, $enum_variant:ident, $range:expr) ),*
        $(,)?
    ) => {
        // This function handles fakers that take a `Range<usize>` argument.
        // Even though `locale` is not passed to the constructor, the `Words<L>` type
        // still has `L` as a type parameter, and its `Fake` impl needs `L` to satisfy
        // the relevant GenFn trait.
        fn get_range_arg_fake_data<L>(data_type: &str, locale: L) -> Option<FakeData>
        where
            L: Data + Copy + 'static // Standard locale bounds
               // All GenFn traits for fakers in this category:
               + WordGenFn + WordsGenFn + SentenceGenFn + SentencesGenFn + ParagraphGenFn + ParagraphsGenFn,
        {
            match data_type {
                $(
                    $string_key => Some(FakeData::$enum_variant( ($faker_type_path)($range).fake::<String>() )),
                )*
                _ => None, // If data_type doesn't match any in this category
            }
        }
    };
}

fn generate_city2() -> String {
    CityName(EN).fake()
    //  (EN).fake::<String>()
    // Faker.fake::<(u8, u32, u32)>()
}

fn gen_fake_data2<L>(data_type: &str, locale: L) -> FakeData {
    match data_type {
        "Name" => FakeData::CityName(CityName(locale).fake()),
        "Age" => FakeData::Age(rand::rng().random_range(8..90)),
        _ => FakeData::Other("Nothing".to_string()),
    }
}
// use fake::locales::Data; // Import the Data trait for generic locale handling

// Now, call the macro with your faker mappings, using the `raw` variants
// Category 1: Fakers that take (locale)
generate_one_arg_fakers! {
    ("FirstName", FirstName, FirstName), ("LastName", LastName, LastName),
    ("Title", Title, Title), ("Suffix", Suffix, Suffix), ("Name", Name, Name),
    ("NameWithTitle", NameWithTitle, NameWithTitle), ("Seniority", Seniority, Seniority),
    ("Field", Field, Field), ("Position", Position, Position),

    ("Semver", Semver, Semver), ("SemverStable", SemverStable, SemverStable),
    ("SemverUnstable", SemverUnstable, SemverUnstable),
    ("Isbn", Isbn, Isbn), ("Isbn10", Isbn10, Isbn10), ("Isbn13", Isbn13, Isbn13),
    ("CreditCardNumber", CreditCardNumber, CreditCardNumber),

    ("CompanySuffix", CompanySuffix, CompanySuffix), ("CompanyName", CompanyName, CompanyName),
    ("Buzzword", Buzzword, Buzzword), ("BuzzwordMiddle", BuzzwordMiddle, BuzzwordMiddle),
    ("BuzzwordTail", BuzzwordTail, BuzzwordTail), ("CatchPhrase", CatchPhrase, CatchPhrase),
    ("BsVerb", BsVerb, BsVerb), ("BsAdj", BsAdj, BsAdj), ("BsNoun", BsNoun, BsNoun), ("Bs", Bs, Bs),
    ("Profession", Profession, Profession), ("Industry", Industry, Industry),

    ("FreeEmailProvider", FreeEmailProvider, FreeEmailProvider), ("DomainSuffix", DomainSuffix, DomainSuffix),
    ("FreeEmail", FreeEmail, FreeEmail), ("SafeEmail", SafeEmail, SafeEmail),
    ("Username", Username, Username), ("Password", Password, Password),
    ("IPv4", IPv4, IPv4), ("IPv6", IPv6, IPv6), ("IP", IP, IP), ("MACAddress", MACAddress, MACAddress),
    ("UserAgent", UserAgent, UserAgent),

    ("FilePath", FilePath, FilePath), ("FileName", FileName, FileName),
    ("FileExtension", FileExtension, FileExtension), ("DirPath", DirPath, DirPath),
    ("MimeType", MimeType, MimeType),

    ("CurrencyCode", CurrencyCode, CurrencyCode), ("CurrencyName", CurrencyName, CurrencyName),
    ("CurrencySymbol", CurrencySymbol, CurrencySymbol), ("Bic", Bic, Bic), ("Isin", Isin, Isin),

    ("HexColor", HexColor, HexColor), ("RgbColor", RgbColor, RgbColor),
    ("RgbaColor", RgbaColor, RgbaColor), ("HslColor", HslColor, HslColor),
    ("HslaColor", HslaColor, HslaColor), ("Color", Color, Color),

    ("Time", Time, Time), ("Date", Date, Date), ("DateTime", DateTime, DateTime),

    ("RfcStatusCode", RfcStatusCode, RfcStatusCode), ("ValidStatusCode", ValidStatusCode, ValidStatusCode),

    ("PhoneNumber", PhoneNumber, PhoneNumber), ("CellNumber", CellNumber, CellNumber),
}

// Category 2: Fakers that take (locale, u8)
generate_two_arg_u8_fakers! {
    ("CityPrefix", CityPrefix, CityPrefix), ("CitySuffix", CitySuffix, CitySuffix),
    ("CityName", CityName, CityName), ("CountryName", CountryName, CountryName),
    ("CountryCode", CountryCode, CountryCode), ("StreetSuffix", StreetSuffix, StreetSuffix),
    ("StreetName", StreetName, StreetName), ("TimeZone", TimeZone, TimeZone),
    ("StateName", StateName, StateName), ("StateAbbr", StateAbbr, StateAbbr),
    ("SecondaryAddressType", SecondaryAddressType, SecondaryAddressType),
    ("SecondaryAddress", SecondaryAddress, SecondaryAddress), ("ZipCode", ZipCode, ZipCode),
    ("PostCode", PostCode, PostCode), ("BuildingNumber", BuildingNumber, BuildingNumber),
    ("Latitude", Latitude, Latitude), ("Longitude", Longitude, Longitude),
    ("Geohash", Geohash, Geohash),
}

// Category 3: Fakers that take (Range<usize>)
generate_range_arg_fakers! {
    ("Word", Word, Word, 0..1), // Single word
    ("Words", Words, Words, 0..10), // 0 to 10 words
    ("Sentence", Sentence, Sentence, 0..1), // Single sentence
    ("Sentences", Sentences, Sentences, 0..10), // 0 to 10 sentences
    ("Paragraph", Paragraph, Paragraph, 0..1), // Single paragraph
    ("Paragraphs", Paragraphs, Paragraphs, 0..10), // 0 to 10 paragraphs
}
fn main() {
    use fake::locales::{EN, FR, PT_BR};

    // The syntax you provided that works for concrete locales:
    println!(
        "Working example (CityName EN): {}",
        CityName(EN).fake::<String>()
    );
    println!(
        "Working example (FirstName EN): {}",
        fake::faker::name::raw::FirstName(EN).fake::<String>()
    );

    // Using the generic function with EN locale:
    let name_en = gen_fake_data("Name", EN);
    match name_en {
        FakeData::Name(s) => println!("Generated Name (EN) via macro: {}", s),
        _ => unreachable!(),
    }

    let city_en = gen_fake_data("CityName", EN);
    match city_en {
        FakeData::CityName(s) => println!("Generated City (EN) via macro: {}", s),
        _ => unreachable!(),
    }

    let age = gen_fake_data("Age", EN);
    match age {
        FakeData::Age(a) => println!("Generated Age: {}", a),
        _ => unreachable!(),
    }

    // Example usage with French locale
    let name_fr = gen_fake_data("Name", FR);
    match name_fr {
        FakeData::Name(s) => println!("Generated Name (FR) via macro: {}", s),
        _ => unreachable!(),
    }

    let city_fr = gen_fake_data("CityName", FR);
    match city_fr {
        FakeData::CityName(s) => println!("Generated City (FR) via macro: {}", s),
        _ => unreachable!(),
    }

    // Example usage with Brazilian Portuguese locale
    let name_pt_br = gen_fake_data("Name", PT_BR);
    match name_pt_br {
        FakeData::Name(s) => println!("Generated Name (PT_BR) via macro: {}", s),
        _ => unreachable!(),
    }

    let unknown = gen_fake_data("NonExistentType", EN);
    match unknown {
        FakeData::Other(s) => println!("Generated Other: {}", s),
        _ => unreachable!(),
    }
}

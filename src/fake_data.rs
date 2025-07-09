use fake::{
    Fake, // Import specific locale instances directly from `fake::locales`
    faker::{
        address::raw::*, barcode::raw::*, chrono::raw::*, company::raw::*, creditcard::raw::*,
        currency::raw::*, filesystem::raw::*, finance::raw::*, internet::raw::*, job::raw::*,
        lorem::raw::*, name::raw::*, phone_number::raw::*,
    },
    locales::{AR_SA, DE_DE, EN, FR_FR, IT_IT, JA_JP, PT_BR, PT_PT, ZH_CN, ZH_TW},
};
use prost_reflect::{Kind as ProstFieldKind, Value as ProstFieldValue};
use rand::{Rng, rngs::ThreadRng};
use serde::Serialize; // Import Serialize trait for JSON serialization
use std::fmt; // Import Display trait for formatting // Import Rng for random number generation

use std::borrow::Cow; // Import Cow for string handling

#[derive(Debug, Clone, Serialize)]
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
    Sentence(String),
    Paragraph(String),
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
    // Vector types for words, sentences, and paragraphs
    Words(Vec<String>),
    Sentences(Vec<String>),
    Paragraphs(Vec<String>),

    // Custom types, still using Faker to generate them
    Age(u32),
    Other(String), // For the default case
}

impl Default for FakeData {
    fn default() -> Self {
        FakeData::Other("".to_string())
    }
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

impl FakeData {
    // Helper to provide owned String for `into_prost_reflect_value` below
    pub fn into_string(self) -> String {
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
            | FakeData::Other(s) => s,
            FakeData::Age(val) => val.to_string(),
            FakeData::Words(v) | FakeData::Sentences(v) | FakeData::Paragraphs(v) => v.join(" "), // Join into single string
        }
    }

    // Helper to get generic Prost field values of any type
    pub fn into_prost_reflect_value(self, expected_kind: &ProstFieldKind) -> ProstFieldValue {
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
            | FakeData::Other(s) => {
                match expected_kind {
                    &ProstFieldKind::String => ProstFieldValue::String(s),
                    &ProstFieldKind::Bytes => ProstFieldValue::Bytes(s.into()),
                    // ProstFieldKind::Bytes => ProstFieldValue::Bytes(self.into_string().into()),
                    _ => {
                        log::warn!(
                            "Mismatched type: FakeData is String, but Prost field is {:?}. Defaulting to String.",
                            expected_kind
                        );
                        ProstFieldValue::String(s) // Fallback for unexpected kinds
                    }
                }
            }
            FakeData::Age(val) => match expected_kind {
                &ProstFieldKind::Int32 => ProstFieldValue::I32(val as i32),
                &ProstFieldKind::Int64 => ProstFieldValue::I64(val as i64),
                &ProstFieldKind::Uint32 => ProstFieldValue::U32(val),
                &ProstFieldKind::Uint64 => ProstFieldValue::U64(val as u64),
                &ProstFieldKind::Sfixed32 => ProstFieldValue::I32(val as i32),
                &ProstFieldKind::Sfixed64 => ProstFieldValue::I64(val as i64),
                &ProstFieldKind::Fixed32 => ProstFieldValue::U32(val),
                &ProstFieldKind::Fixed64 => ProstFieldValue::U64(val as u64),
                _ => {
                    log::warn!(
                        "Mismatched type: FakeData is U32, but Prost field is {:?}. Defaulting to U32.",
                        expected_kind
                    );
                    ProstFieldValue::U32(val)
                }
            },
            FakeData::Words(v) | FakeData::Sentences(v) | FakeData::Paragraphs(v) => {
                // For repeated string fields, we provide a Vec<Value> where each is a String
                if expected_kind == &ProstFieldKind::String {
                    ProstFieldValue::List(v.into_iter().map(ProstFieldValue::String).collect())
                } else {
                    log::warn!(
                        "Mismatched type: FakeData is Vec<String>, but Prost field is {:?}. Joining into single String Value.",
                        expected_kind
                    );
                    ProstFieldValue::String(v.join(", "))
                }
            }
        }
    }
    // Helper to get string data
    pub fn as_str_cow<'a>(&'a self) -> Option<Cow<'a, str>> {
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
            | FakeData::Other(s) => Some(Cow::Borrowed(s)),
            _ => None,
        }
    }
    // Helper to get numeric data
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            FakeData::Age(i) => Some(*i),
            _ => None,
        }
    }
    // Helper to get <Vec<String>> data
    pub fn as_vec_string(&self) -> Option<&Vec<String>> {
        match self {
            FakeData::Words(v) | FakeData::Sentences(v) | FakeData::Paragraphs(v) => Some(v),
            _ => None,
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
            "ar_sa" | "ar" | "arabic" => {
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
    // If no language is provided, default to English
    let derived_language = if language.is_empty() {
        "english"
    } else {
        language
    };
    let result = generate_faker_match_arms!(
        data_type,
        derived_language,
        // --- START OF FIRST LIST (Unit Struct Fakers - call method directly) ---
        [
            ("FirstName", FirstName, FirstName),
            ("LastName", LastName, LastName),
            ("Suffix", Suffix, Suffix),
            ("Name", Name, Name),
            ("NameWithTitle", NameWithTitle, NameWithTitle),
            ("CreditCardNumber", CreditCardNumber, CreditCardNumber),
            ("CompanySuffix", CompanySuffix, CompanySuffix),
            ("CompanyName", CompanyName, CompanyName),
            ("Buzzword", Buzzword, Buzzword),
            ("BuzzwordMiddle", BuzzwordMiddle, BuzzwordMiddle),
            ("BuzzwordTail", BuzzwordTail, BuzzwordTail),
            ("CatchPhrase", CatchPhrase, CatchPhrase),
            ("BsVerb", BsVerb, BsVerb),
            ("BsAdj", BsAdj, BsAdj),
            ("BsNoun", BsNoun, BsNoun),
            ("Bs", Bs, Bs),
            ("Profession", Profession, Profession),
            ("Industry", Industry, Industry),
            ("FreeEmailProvider", FreeEmailProvider, FreeEmailProvider),
            ("DomainSuffix", DomainSuffix, DomainSuffix),
            ("FreeEmail", FreeEmail, FreeEmail),
            ("SafeEmail", SafeEmail, SafeEmail),
            ("Username", Username, Username),
            ("IPv4", IPv4, IPv4),
            ("IPv6", IPv6, IPv6),
            ("IP", IP, IP),
            ("MACAddress", MACAddress, MACAddress),
            ("UserAgent", UserAgent, UserAgent),
            ("FilePath", FilePath, FilePath),
            ("FileName", FileName, FileName),
            ("FileExtension", FileExtension, FileExtension),
            ("DirPath", DirPath, DirPath),
            ("MimeType", MimeType, MimeType),
            ("CurrencyCode", CurrencyCode, CurrencyCode),
            ("CurrencyName", CurrencyName, CurrencyName),
            ("CurrencySymbol", CurrencySymbol, CurrencySymbol),
            ("Bic", Bic, Bic),
            ("Isin", Isin, Isin),
            ("Time", Time, Time),
            ("Date", Date, Date),
            ("DateTime", DateTime, DateTime),
            ("PhoneNumber", PhoneNumber, PhoneNumber),
            ("CellNumber", CellNumber, CellNumber),
            ("CityPrefix", CityPrefix, CityPrefix),
            ("CitySuffix", CitySuffix, CitySuffix),
            ("CityName", CityName, CityName),
            ("CountryName", CountryName, CountryName),
            ("CountryCode", CountryCode, CountryCode),
            ("StreetSuffix", StreetSuffix, StreetSuffix),
            ("TimeZone", TimeZone, TimeZone),
            ("StateName", StateName, StateName),
            ("StateAbbr", StateAbbr, StateAbbr),
            (
                "SecondaryAddressType",
                SecondaryAddressType,
                SecondaryAddressType
            ),
            ("SecondaryAddress", SecondaryAddress, SecondaryAddress),
            ("ZipCode", ZipCode, ZipCode),
            ("PostCode", PostCode, PostCode),
            ("BuildingNumber", BuildingNumber, BuildingNumber),
            ("Latitude", Latitude, Latitude),
            ("Longitude", Longitude, Longitude),
            ("Word", Word, Word),
            ("Seniority", Seniority, Seniority),
            ("Field", Field, Field),
            ("Position", Position, Position),
            ("Semver", Semver, Semver),
            ("SemverStable", SemverStable, SemverStable),
            ("SemverUnstable", SemverUnstable, SemverUnstable),
            ("Isbn", Isbn, Isbn),
            ("Isbn10", Isbn10, Isbn10),
            ("Isbn13", Isbn13, Isbn13)
        ],
        // --- START OF SECOND LIST (Fakers with Range<usize> constructor arguments) ---
        [
            ("Words", Words, Words, 0..10),
            ("Sentence", Sentence, Sentence, 5..10),
            ("Sentences", Sentences, Sentences, 0..10),
            ("Paragraph", Paragraph, Paragraph, 5..10),
            ("Paragraphs", Paragraphs, Paragraphs, 0..10),
            ("Password", Password, Password, 10..20)
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

#[cfg(test)] // This attribute tells Cargo to compile and run the test only when `cargo test` is run.
mod fake_data_tests {
    use super::*; // Import everything from the outer scope (the crate root)

    /// Test `get_fake_data` with a known string data type and default language.
    #[test]
    fn test_get_fake_data_first_name_en() {
        let fake_name = get_fake_data("FirstName", "en");
        assert!(fake_name.is_some()); // Assert that a value was returned
        if let Some(FakeData::FirstName(name)) = fake_name {
            assert!(!name.is_empty()); // Assert the generated name is not empty
        // You could add more specific checks here if you knew the expected format,
        // e.g., regex for names, but for random data, non-empty is a good start.
        } else {
            panic!("Expected FakeData::FirstName variant"); // Fail if it's not the expected variant
        }
    }

    /// Test `get_fake_data` with a numeric data type ("Age").
    #[test]
    fn test_get_fake_data_age() {
        let fake_age = get_fake_data("Age", "en");
        assert!(fake_age.is_some());
        if let Some(FakeData::Age(age)) = fake_age {
            assert!(age >= 8 && age <= 90); // Check if age is within the expected range
        } else {
            panic!("Expected FakeData::Age variant");
        }
    }

    /// Test `get_fake_data` with a list-based data type ("Words").
    #[test]
    fn test_get_fake_data_words() {
        let fake_words = get_fake_data("Words", "en");
        assert!(fake_words.is_some());
        if let Some(FakeData::Words(words)) = fake_words {
            assert!(!words.is_empty()); // Ensure the list of words is not empty
            assert!(words.len() <= 10); // Check if the number of words is within the range 0..10
            for word in words {
                assert!(!word.is_empty()); // Ensure each word is not empty
            }
        } else {
            panic!("Expected FakeData::Words variant");
        }
    }

    /// Test `get_fake_data` with an unsupported data type.
    #[test]
    fn test_get_fake_data_unsupported_type() {
        let fake_data = get_fake_data("UnsupportedType", "en");
        assert!(fake_data.is_none()); // Expect None for unsupported types
    }

    /// Test `get_fake_data` with a different language (German).
    #[test]
    fn test_get_fake_data_city_de() {
        let fake_city = get_fake_data("CityName", "de");
        assert!(fake_city.is_some());
        if let Some(FakeData::CityName(city)) = fake_city {
            assert!(!city.is_empty());
            // More specific checks for German city names could be added if needed
        } else {
            panic!("Expected FakeData::CityName variant");
        }
    }

    /// Test `FakeData::into_string` for a simple string type.
    #[test]
    fn test_fake_data_into_string_city_prefix() {
        let fake_data = FakeData::CityPrefix("North".to_string());
        assert_eq!(fake_data.into_string(), "North".to_string());
    }

    /// Test `FakeData::into_string` for an Age type.
    #[test]
    fn test_fake_data_into_string_age() {
        let fake_data = FakeData::Age(30);
        assert_eq!(fake_data.into_string(), "30".to_string());
    }

    /// Test `FakeData::into_string` for a Words type.
    #[test]
    fn test_fake_data_into_string_words() {
        let fake_data = FakeData::Words(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(fake_data.into_string(), "hello world".to_string());
    }

    /// Test `FakeData::into_prost_reflect_value` for a String kind.
    #[test]
    fn test_fake_data_into_prost_reflect_value_string() {
        let fake_data = FakeData::FirstName("John".to_string());
        let prost_value = fake_data.into_prost_reflect_value(&ProstFieldKind::String);
        assert_eq!(prost_value, ProstFieldValue::String("John".to_string()));
    }

    /// Test `FakeData::into_prost_reflect_value` for an Int32 kind with Age.
    #[test]
    fn test_fake_data_into_prost_reflect_value_int32() {
        let fake_data = FakeData::Age(25);
        let prost_value = fake_data.into_prost_reflect_value(&ProstFieldKind::Int32);
        assert_eq!(prost_value, ProstFieldValue::I32(25));
    }

    /// Test `FakeData::into_prost_reflect_value` for a List of Strings.
    #[test]
    fn test_fake_data_into_prost_reflect_value_list_string() {
        let fake_data = FakeData::Words(vec!["one".to_string(), "two".to_string()]);
        let prost_value = fake_data.into_prost_reflect_value(&ProstFieldKind::String);
        assert_eq!(
            prost_value,
            ProstFieldValue::List(vec![
                ProstFieldValue::String("one".to_string()),
                ProstFieldValue::String("two".to_string()),
            ])
        );
    }
}

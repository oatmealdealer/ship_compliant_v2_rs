mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
#[doc = r" Types used as operation parameters and responses."]
#[allow(clippy::all)]
pub mod types {
    #[doc = r" Error types."]
    pub mod error {
        #[doc = r" Error from a TryFrom or FromStr implementation."]
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    #[doc = "Address"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Address\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"city\","]
    #[doc = "    \"firstName\","]
    #[doc = "    \"lastName\","]
    #[doc = "    \"state\","]
    #[doc = "    \"street1\","]
    #[doc = "    \"zip1\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"company\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"country\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"dateOfBirth\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"email\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fax\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"firstName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"lastName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"phone\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Address {
        pub city: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fax: Option<String>,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        pub state: String,
        pub street1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        pub zip1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&Address> for Address {
        fn from(value: &Address) -> Self {
            value.clone()
        }
    }

    #[doc = "AddressDetails"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressDetails\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cityAbbreviation\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"congressionalDistrict\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"countyFips\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeZone\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeZoneCode\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddressDetails {
        #[serde(
            rename = "cityAbbreviation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub city_abbreviation: Option<String>,
        #[serde(
            rename = "congressionalDistrict",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub congressional_district: Option<String>,
        #[serde(
            rename = "countyFips",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub county_fips: Option<String>,
        #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
        pub time_zone: Option<String>,
        #[serde(
            rename = "timeZoneCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_zone_code: Option<String>,
    }

    impl From<&AddressDetails> for AddressDetails {
        fn from(value: &AddressDetails) -> Self {
            value.clone()
        }
    }

    #[doc = "AddressOption"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressOption\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"ignoreStreetLevelErrors\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"rejectIfAddressSuggested\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddressOption {
        #[serde(
            rename = "ignoreStreetLevelErrors",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ignore_street_level_errors: Option<bool>,
        #[serde(
            rename = "rejectIfAddressSuggested",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reject_if_address_suggested: Option<bool>,
    }

    impl From<&AddressOption> for AddressOption {
        fn from(value: &AddressOption) -> Self {
            value.clone()
        }
    }

    #[doc = "AddressParts"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressParts\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"company\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"mailBoxName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"mailBoxNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"postDirection\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"preDirection\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetSuffix\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"suiteName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"suiteNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddressParts {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(
            rename = "mailBoxName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mail_box_name: Option<String>,
        #[serde(
            rename = "mailBoxNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mail_box_number: Option<String>,
        #[serde(
            rename = "postDirection",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub post_direction: Option<String>,
        #[serde(
            rename = "preDirection",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pre_direction: Option<String>,
        #[serde(
            rename = "streetName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_name: Option<String>,
        #[serde(
            rename = "streetNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_number: Option<String>,
        #[serde(
            rename = "streetSuffix",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_suffix: Option<String>,
        #[serde(rename = "suiteName", default, skip_serializing_if = "Option::is_none")]
        pub suite_name: Option<String>,
        #[serde(
            rename = "suiteNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub suite_number: Option<String>,
    }

    impl From<&AddressParts> for AddressParts {
        fn from(value: &AddressParts) -> Self {
            value.clone()
        }
    }

    #[doc = "AddressStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressStatus\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"AddressOutOfRange\","]
    #[doc = "    \"AddressSuggested\","]
    #[doc = "    \"ComponentMismatch\","]
    #[doc = "    \"MultipleMatches\","]
    #[doc = "    \"NonDeliverableAddress\","]
    #[doc = "    \"NoStreetData\","]
    #[doc = "    \"UnknownStreet\","]
    #[doc = "    \"Validated\","]
    #[doc = "    \"ZipCodeDoesNotExist\","]
    #[doc = "    \"ZipCodeDoesNotMatchCityState\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AddressStatus {
        Null,
        AddressOutOfRange,
        AddressSuggested,
        ComponentMismatch,
        MultipleMatches,
        NonDeliverableAddress,
        NoStreetData,
        UnknownStreet,
        Validated,
        ZipCodeDoesNotExist,
        ZipCodeDoesNotMatchCityState,
    }

    impl From<&AddressStatus> for AddressStatus {
        fn from(value: &AddressStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for AddressStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::AddressOutOfRange => "AddressOutOfRange".to_string(),
                Self::AddressSuggested => "AddressSuggested".to_string(),
                Self::ComponentMismatch => "ComponentMismatch".to_string(),
                Self::MultipleMatches => "MultipleMatches".to_string(),
                Self::NonDeliverableAddress => "NonDeliverableAddress".to_string(),
                Self::NoStreetData => "NoStreetData".to_string(),
                Self::UnknownStreet => "UnknownStreet".to_string(),
                Self::Validated => "Validated".to_string(),
                Self::ZipCodeDoesNotExist => "ZipCodeDoesNotExist".to_string(),
                Self::ZipCodeDoesNotMatchCityState => "ZipCodeDoesNotMatchCityState".to_string(),
            }
        }
    }

    impl std::str::FromStr for AddressStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "AddressOutOfRange" => Ok(Self::AddressOutOfRange),
                "AddressSuggested" => Ok(Self::AddressSuggested),
                "ComponentMismatch" => Ok(Self::ComponentMismatch),
                "MultipleMatches" => Ok(Self::MultipleMatches),
                "NonDeliverableAddress" => Ok(Self::NonDeliverableAddress),
                "NoStreetData" => Ok(Self::NoStreetData),
                "UnknownStreet" => Ok(Self::UnknownStreet),
                "Validated" => Ok(Self::Validated),
                "ZipCodeDoesNotExist" => Ok(Self::ZipCodeDoesNotExist),
                "ZipCodeDoesNotMatchCityState" => Ok(Self::ZipCodeDoesNotMatchCityState),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "AddressSuggestion"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressSuggestion\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"details\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressDetails\""]
    #[doc = "    },"]
    #[doc = "    \"parts\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressParts\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AddressSuggestion {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<AddressDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parts: Option<AddressParts>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&AddressSuggestion> for AddressSuggestion {
        fn from(value: &AddressSuggestion) -> Self {
            value.clone()
        }
    }

    #[doc = "CheckAndCommitSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CheckAndCommitSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"salesOrder\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"commitOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"persistOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CheckAndCommitSalesOrder {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<AddressOption>,
        #[serde(
            rename = "commitOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub commit_option: Option<String>,
        #[serde(
            rename = "persistOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub persist_option: Option<String>,
        #[serde(rename = "salesOrder")]
        pub sales_order: SalesOrder,
    }

    impl From<&CheckAndCommitSalesOrder> for CheckAndCommitSalesOrder {
        fn from(value: &CheckAndCommitSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "CheckComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CheckComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesOrderComplianceTaxResponse\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CheckComplianceResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::check_compliance_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<SalesOrderComplianceTaxResponse>,
        #[serde(
            rename = "statusCode",
            default = "defaults::check_compliance_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&CheckComplianceResponse> for CheckComplianceResponse {
        fn from(value: &CheckComplianceResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "ComplianceDetailResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComplianceDetailResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ComplianceDetailType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"BrandNameNotRegistered\","]
    #[doc = "        \"LabelNotRegistered\","]
    #[doc = "        \"NextShipDate\","]
    #[doc = "        \"VolumeOverLimit\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"value\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ComplianceDetailResponse {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<ComplianceDetailType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&ComplianceDetailResponse> for ComplianceDetailResponse {
        fn from(value: &ComplianceDetailResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "ComplianceDetailType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComplianceDetailType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"BrandNameNotRegistered\","]
    #[doc = "    \"LabelNotRegistered\","]
    #[doc = "    \"NextShipDate\","]
    #[doc = "    \"VolumeOverLimit\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ComplianceDetailType {
        BrandNameNotRegistered,
        LabelNotRegistered,
        NextShipDate,
        VolumeOverLimit,
    }

    impl From<&ComplianceDetailType> for ComplianceDetailType {
        fn from(value: &ComplianceDetailType) -> Self {
            value.clone()
        }
    }

    impl ToString for ComplianceDetailType {
        fn to_string(&self) -> String {
            match *self {
                Self::BrandNameNotRegistered => "BrandNameNotRegistered".to_string(),
                Self::LabelNotRegistered => "LabelNotRegistered".to_string(),
                Self::NextShipDate => "NextShipDate".to_string(),
                Self::VolumeOverLimit => "VolumeOverLimit".to_string(),
            }
        }
    }

    impl std::str::FromStr for ComplianceDetailType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "BrandNameNotRegistered" => Ok(Self::BrandNameNotRegistered),
                "LabelNotRegistered" => Ok(Self::LabelNotRegistered),
                "NextShipDate" => Ok(Self::NextShipDate),
                "VolumeOverLimit" => Ok(Self::VolumeOverLimit),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "CustomerAggregateVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CustomerAggregateVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"customerType\": {"]
    #[doc = "      \"title\": \"CustomerType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Household\","]
    #[doc = "        \"Individual\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeFrameCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"timeFrameType\": {"]
    #[doc = "      \"title\": \"TimeFrameType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Calendar\","]
    #[doc = "        \"Rolling\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeFrameUnit\": {"]
    #[doc = "      \"title\": \"TimeFrameUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Minutes\","]
    #[doc = "        \"Hours\","]
    #[doc = "        \"Days\","]
    #[doc = "        \"Weeks\","]
    #[doc = "        \"Months\","]
    #[doc = "        \"Years\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitOfMeasure\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CustomerAggregateVolumeLimitDetail {
        #[serde(
            rename = "customerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_type: Option<CustomerType>,
        #[serde(
            rename = "timeFrameCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_count: Option<i32>,
        #[serde(
            rename = "timeFrameType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_type: Option<TimeFrameType>,
        #[serde(
            rename = "timeFrameUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_unit: Option<TimeFrameUnit>,
        #[serde(
            rename = "unitOfMeasure",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub unit_of_measure: Option<VolumeUnit>,
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
    }

    impl From<&CustomerAggregateVolumeLimitDetail> for CustomerAggregateVolumeLimitDetail {
        fn from(value: &CustomerAggregateVolumeLimitDetail) -> Self {
            value.clone()
        }
    }

    #[doc = "CustomerType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CustomerType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Household\","]
    #[doc = "    \"Individual\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CustomerType {
        Null,
        Household,
        Individual,
    }

    impl From<&CustomerType> for CustomerType {
        fn from(value: &CustomerType) -> Self {
            value.clone()
        }
    }

    impl ToString for CustomerType {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Household => "Household".to_string(),
                Self::Individual => "Individual".to_string(),
            }
        }
    }

    impl std::str::FromStr for CustomerType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Household" => Ok(Self::Household),
                "Individual" => Ok(Self::Individual),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "For Quote Salestax we need to suggest an Address with the Error Message"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorData\","]
    #[doc = "  \"description\": \"For Quote Salestax we need to suggest an Address with the Error Message\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressSuggestion\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressSuggestion\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ErrorData {
        #[serde(
            rename = "addressSuggestion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_suggestion: Option<AddressSuggestion>,
    }

    impl From<&ErrorData> for ErrorData {
        fn from(value: &ErrorData) -> Self {
            value.clone()
        }
    }

    #[doc = "ErrorTarget"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorTarget\","]
    #[doc = "  \"default\": \"SalesOrder\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Address\","]
    #[doc = "    \"Age\","]
    #[doc = "    \"Batch\","]
    #[doc = "    \"Brand\","]
    #[doc = "    \"Distributor\","]
    #[doc = "    \"Product\","]
    #[doc = "    \"SalesOrder\","]
    #[doc = "    \"Shipment\","]
    #[doc = "    \"Tax\","]
    #[doc = "    \"SSO\","]
    #[doc = "    \"License\","]
    #[doc = "    \"Tracking\","]
    #[doc = "    \"HoldLocation\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ErrorTarget {
        Null,
        Address,
        Age,
        Batch,
        Brand,
        Distributor,
        Product,
        SalesOrder,
        Shipment,
        Tax,
        #[serde(rename = "SSO")]
        Sso,
        License,
        Tracking,
        HoldLocation,
    }

    impl From<&ErrorTarget> for ErrorTarget {
        fn from(value: &ErrorTarget) -> Self {
            value.clone()
        }
    }

    impl ToString for ErrorTarget {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Address => "Address".to_string(),
                Self::Age => "Age".to_string(),
                Self::Batch => "Batch".to_string(),
                Self::Brand => "Brand".to_string(),
                Self::Distributor => "Distributor".to_string(),
                Self::Product => "Product".to_string(),
                Self::SalesOrder => "SalesOrder".to_string(),
                Self::Shipment => "Shipment".to_string(),
                Self::Tax => "Tax".to_string(),
                Self::Sso => "SSO".to_string(),
                Self::License => "License".to_string(),
                Self::Tracking => "Tracking".to_string(),
                Self::HoldLocation => "HoldLocation".to_string(),
            }
        }
    }

    impl std::str::FromStr for ErrorTarget {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Address" => Ok(Self::Address),
                "Age" => Ok(Self::Age),
                "Batch" => Ok(Self::Batch),
                "Brand" => Ok(Self::Brand),
                "Distributor" => Ok(Self::Distributor),
                "Product" => Ok(Self::Product),
                "SalesOrder" => Ok(Self::SalesOrder),
                "Shipment" => Ok(Self::Shipment),
                "Tax" => Ok(Self::Tax),
                "SSO" => Ok(Self::Sso),
                "License" => Ok(Self::License),
                "Tracking" => Ok(Self::Tracking),
                "HoldLocation" => Ok(Self::HoldLocation),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ErrorTarget {
        fn default() -> Self {
            ErrorTarget::SalesOrder
        }
    }

    #[doc = "ErrorType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorType\","]
    #[doc = "  \"default\": \"Validation\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Authentication\","]
    #[doc = "    \"Server\","]
    #[doc = "    \"Validation\","]
    #[doc = "    \"Request\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ErrorType {
        Authentication,
        Server,
        Validation,
        Request,
    }

    impl From<&ErrorType> for ErrorType {
        fn from(value: &ErrorType) -> Self {
            value.clone()
        }
    }

    impl ToString for ErrorType {
        fn to_string(&self) -> String {
            match *self {
                Self::Authentication => "Authentication".to_string(),
                Self::Server => "Server".to_string(),
                Self::Validation => "Validation".to_string(),
                Self::Request => "Request".to_string(),
            }
        }
    }

    impl std::str::FromStr for ErrorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Authentication" => Ok(Self::Authentication),
                "Server" => Ok(Self::Server),
                "Validation" => Ok(Self::Validation),
                "Request" => Ok(Self::Request),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ErrorType {
        fn default() -> Self {
            ErrorType::Validation
        }
    }

    #[doc = "FreightSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FreightSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRate\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FreightSalesTaxRate {
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "salesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rate: Option<f64>,
    }

    impl From<&FreightSalesTaxRate> for FreightSalesTaxRate {
        fn from(value: &FreightSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "FulfillmentExceptionTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FulfillmentExceptionTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"None\","]
    #[doc = "    \"Inventory\","]
    #[doc = "    \"NonCompliant\","]
    #[doc = "    \"Other\","]
    #[doc = "    \"Updated\","]
    #[doc = "    \"Setup\","]
    #[doc = "    \"Temperature\","]
    #[doc = "    \"Voided\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FulfillmentExceptionTypes {
        Null,
        None,
        Inventory,
        NonCompliant,
        Other,
        Updated,
        Setup,
        Temperature,
        Voided,
    }

    impl From<&FulfillmentExceptionTypes> for FulfillmentExceptionTypes {
        fn from(value: &FulfillmentExceptionTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for FulfillmentExceptionTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::None => "None".to_string(),
                Self::Inventory => "Inventory".to_string(),
                Self::NonCompliant => "NonCompliant".to_string(),
                Self::Other => "Other".to_string(),
                Self::Updated => "Updated".to_string(),
                Self::Setup => "Setup".to_string(),
                Self::Temperature => "Temperature".to_string(),
                Self::Voided => "Voided".to_string(),
            }
        }
    }

    impl std::str::FromStr for FulfillmentExceptionTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "None" => Ok(Self::None),
                "Inventory" => Ok(Self::Inventory),
                "NonCompliant" => Ok(Self::NonCompliant),
                "Other" => Ok(Self::Other),
                "Updated" => Ok(Self::Updated),
                "Setup" => Ok(Self::Setup),
                "Temperature" => Ok(Self::Temperature),
                "Voided" => Ok(Self::Voided),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "FulfillmentType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FulfillmentType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Club\","]
    #[doc = "    \"Daily\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FulfillmentType {
        Null,
        Club,
        Daily,
    }

    impl From<&FulfillmentType> for FulfillmentType {
        fn from(value: &FulfillmentType) -> Self {
            value.clone()
        }
    }

    impl ToString for FulfillmentType {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Club => "Club".to_string(),
                Self::Daily => "Daily".to_string(),
            }
        }
    }

    impl std::str::FromStr for FulfillmentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Club" => Ok(Self::Club),
                "Daily" => Ok(Self::Daily),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "GetSalesOrderSuccessResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetSalesOrderSuccessResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceResults\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesOrderComplianceTaxResponse\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesOrderOutput\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetSalesOrderSuccessResponse {
        #[serde(
            rename = "complianceResults",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_results: Option<SalesOrderComplianceTaxResponse>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::get_sales_order_success_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<SalesOrderOutput>,
        #[serde(
            rename = "statusCode",
            default = "defaults::get_sales_order_success_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&GetSalesOrderSuccessResponse> for GetSalesOrderSuccessResponse {
        fn from(value: &GetSalesOrderSuccessResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "LicenseRelationshipTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"LicenseRelationshipTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Default\","]
    #[doc = "    \"Pickup\","]
    #[doc = "    \"RetailerToConsumer\","]
    #[doc = "    \"RetailerToThreeTier\","]
    #[doc = "    \"SupplierToConsumer\","]
    #[doc = "    \"SupplierToThreeTier\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum LicenseRelationshipTypes {
        Default,
        Pickup,
        RetailerToConsumer,
        RetailerToThreeTier,
        SupplierToConsumer,
        SupplierToThreeTier,
    }

    impl From<&LicenseRelationshipTypes> for LicenseRelationshipTypes {
        fn from(value: &LicenseRelationshipTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for LicenseRelationshipTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Default => "Default".to_string(),
                Self::Pickup => "Pickup".to_string(),
                Self::RetailerToConsumer => "RetailerToConsumer".to_string(),
                Self::RetailerToThreeTier => "RetailerToThreeTier".to_string(),
                Self::SupplierToConsumer => "SupplierToConsumer".to_string(),
                Self::SupplierToThreeTier => "SupplierToThreeTier".to_string(),
            }
        }
    }

    impl std::str::FromStr for LicenseRelationshipTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Default" => Ok(Self::Default),
                "Pickup" => Ok(Self::Pickup),
                "RetailerToConsumer" => Ok(Self::RetailerToConsumer),
                "RetailerToThreeTier" => Ok(Self::RetailerToThreeTier),
                "SupplierToConsumer" => Ok(Self::SupplierToConsumer),
                "SupplierToThreeTier" => Ok(Self::SupplierToThreeTier),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "OrderTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"OrderTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Club\","]
    #[doc = "    \"Fax\","]
    #[doc = "    \"InPerson\","]
    #[doc = "    \"Internet\","]
    #[doc = "    \"Mail\","]
    #[doc = "    \"Phone\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum OrderTypes {
        Club,
        Fax,
        InPerson,
        Internet,
        Mail,
        Phone,
    }

    impl From<&OrderTypes> for OrderTypes {
        fn from(value: &OrderTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for OrderTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Club => "Club".to_string(),
                Self::Fax => "Fax".to_string(),
                Self::InPerson => "InPerson".to_string(),
                Self::Internet => "Internet".to_string(),
                Self::Mail => "Mail".to_string(),
                Self::Phone => "Phone".to_string(),
            }
        }
    }

    impl std::str::FromStr for OrderTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Club" => Ok(Self::Club),
                "Fax" => Ok(Self::Fax),
                "InPerson" => Ok(Self::InPerson),
                "Internet" => Ok(Self::Internet),
                "Mail" => Ok(Self::Mail),
                "Phone" => Ok(Self::Phone),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Package"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Package\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"trackingNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Package {
        #[serde(
            rename = "trackingNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_number: Option<String>,
    }

    impl From<&Package> for Package {
        fn from(value: &Package) -> Self {
            value.clone()
        }
    }

    #[doc = "PackageOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PackageOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"trackingNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"trackingStatus\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PackageOutput {
        #[serde(
            rename = "trackingNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_number: Option<String>,
        #[serde(
            rename = "trackingStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_status: Option<String>,
    }

    impl From<&PackageOutput> for PackageOutput {
        fn from(value: &PackageOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "Payment"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Payment\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"subType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"transactionID\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Payment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
        #[serde(
            rename = "transactionID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub transaction_id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&Payment> for Payment {
        fn from(value: &Payment) -> Self {
            value.clone()
        }
    }

    #[doc = "PaymentOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PaymentOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"subType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"transactionID\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"PaymentTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Cash\","]
    #[doc = "        \"Check\","]
    #[doc = "        \"Creditcard\","]
    #[doc = "        \"Giftcard\","]
    #[doc = "        \"Giftcertificate\","]
    #[doc = "        \"Invoice\","]
    #[doc = "        \"Moneyorder\","]
    #[doc = "        \"Other\","]
    #[doc = "        \"Storeaccount\","]
    #[doc = "        \"Travelerscheck\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PaymentOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
        #[serde(
            rename = "transactionID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub transaction_id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<PaymentTypes>,
    }

    impl From<&PaymentOutput> for PaymentOutput {
        fn from(value: &PaymentOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "PaymentTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PaymentTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Cash\","]
    #[doc = "    \"Check\","]
    #[doc = "    \"Creditcard\","]
    #[doc = "    \"Giftcard\","]
    #[doc = "    \"Giftcertificate\","]
    #[doc = "    \"Invoice\","]
    #[doc = "    \"Moneyorder\","]
    #[doc = "    \"Other\","]
    #[doc = "    \"Storeaccount\","]
    #[doc = "    \"Travelerscheck\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PaymentTypes {
        Cash,
        Check,
        Creditcard,
        Giftcard,
        Giftcertificate,
        Invoice,
        Moneyorder,
        Other,
        Storeaccount,
        Travelerscheck,
    }

    impl From<&PaymentTypes> for PaymentTypes {
        fn from(value: &PaymentTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for PaymentTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Cash => "Cash".to_string(),
                Self::Check => "Check".to_string(),
                Self::Creditcard => "Creditcard".to_string(),
                Self::Giftcard => "Giftcard".to_string(),
                Self::Giftcertificate => "Giftcertificate".to_string(),
                Self::Invoice => "Invoice".to_string(),
                Self::Moneyorder => "Moneyorder".to_string(),
                Self::Other => "Other".to_string(),
                Self::Storeaccount => "Storeaccount".to_string(),
                Self::Travelerscheck => "Travelerscheck".to_string(),
            }
        }
    }

    impl std::str::FromStr for PaymentTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Cash" => Ok(Self::Cash),
                "Check" => Ok(Self::Check),
                "Creditcard" => Ok(Self::Creditcard),
                "Giftcard" => Ok(Self::Giftcard),
                "Giftcertificate" => Ok(Self::Giftcertificate),
                "Invoice" => Ok(Self::Invoice),
                "Moneyorder" => Ok(Self::Moneyorder),
                "Other" => Ok(Self::Other),
                "Storeaccount" => Ok(Self::Storeaccount),
                "Travelerscheck" => Ok(Self::Travelerscheck),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "PerBottleVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PerBottleVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PerBottleVolumeLimitDetail {
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<VolumeUnit>,
    }

    impl From<&PerBottleVolumeLimitDetail> for PerBottleVolumeLimitDetail {
        fn from(value: &PerBottleVolumeLimitDetail) -> Self {
            value.clone()
        }
    }

    #[doc = "PerShipmentVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PerShipmentVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PerShipmentVolumeLimitDetail {
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<VolumeUnit>,
    }

    impl From<&PerShipmentVolumeLimitDetail> for PerShipmentVolumeLimitDetail {
        fn from(value: &PerShipmentVolumeLimitDetail) -> Self {
            value.clone()
        }
    }

    #[doc = "ProductSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRate\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ProductSalesTaxRate {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(
            rename = "productKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_key: Option<String>,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "salesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rate: Option<f64>,
    }

    impl From<&ProductSalesTaxRate> for ProductSalesTaxRate {
        fn from(value: &ProductSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "QuoteSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"billTo\","]
    #[doc = "    \"customerKey\","]
    #[doc = "    \"orderType\","]
    #[doc = "    \"purchaseDate\","]
    #[doc = "    \"shipments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Payment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Shipment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct QuoteSalesOrder {
        #[serde(rename = "billTo")]
        pub bill_to: Address,
        #[serde(rename = "customerKey")]
        pub customer_key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<SalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<String>,
        #[serde(rename = "orderType")]
        pub order_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<Payment>>,
        #[serde(rename = "purchaseDate")]
        pub purchase_date: chrono::DateTime<chrono::offset::Utc>,
        #[serde(
            rename = "salesOrderKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order_key: Option<String>,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        pub shipments: Vec<Shipment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl From<&QuoteSalesOrder> for QuoteSalesOrder {
        fn from(value: &QuoteSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "QuoteSalesOrderInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteSalesOrderInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/QuoteSalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct QuoteSalesOrderInput {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<AddressOption>,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<QuoteSalesOrder>,
    }

    impl From<&QuoteSalesOrderInput> for QuoteSalesOrderInput {
        fn from(value: &QuoteSalesOrderInput) -> Self {
            value.clone()
        }
    }

    #[doc = "QuoteTaxForSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteTaxForSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"orderItems\","]
    #[doc = "    \"shipToAddress\","]
    #[doc = "    \"taxSaleType\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"effectiveDate\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"orderItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemTaxCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentItemForTax\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipToAddress\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/TaxRateAddressInput\""]
    #[doc = "    },"]
    #[doc = "    \"shippingAndHandlingCollected\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"taxSaleType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct QuoteTaxForSalesOrder {
        #[serde(
            rename = "effectiveDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub effective_date: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "orderItems")]
        pub order_items: Vec<ShipmentItemForTax>,
        #[serde(rename = "shipToAddress")]
        pub ship_to_address: TaxRateAddressInput,
        #[serde(
            rename = "shippingAndHandlingCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_and_handling_collected: Option<f64>,
        #[serde(rename = "taxSaleType")]
        pub tax_sale_type: String,
    }

    impl From<&QuoteTaxForSalesOrder> for QuoteTaxForSalesOrder {
        fn from(value: &QuoteTaxForSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "QuoteTaxSalesOrderResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteTaxSalesOrderResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"crvSupplementalFees\": {"]
    #[doc = "      \"title\": \"IList`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/RetailDeliveryFees\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"processedAddress\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/TaxRateAddressOutput\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct QuoteTaxSalesOrderResponse {
        #[serde(
            rename = "crvSupplementalFees",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub crv_supplemental_fees: Option<Vec<RetailDeliveryFees>>,
        #[serde(
            rename = "processedAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub processed_address: Option<TaxRateAddressOutput>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::quote_tax_sales_order_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "statusCode",
            default = "defaults::quote_tax_sales_order_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&QuoteTaxSalesOrderResponse> for QuoteTaxSalesOrderResponse {
        fn from(value: &QuoteTaxSalesOrderResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "ResponseStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ResponseStatus\","]
    #[doc = "  \"default\": \"Success\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Failure\","]
    #[doc = "    \"Success\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ResponseStatus {
        Failure,
        Success,
    }

    impl From<&ResponseStatus> for ResponseStatus {
        fn from(value: &ResponseStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for ResponseStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Failure => "Failure".to_string(),
                Self::Success => "Success".to_string(),
            }
        }
    }

    impl std::str::FromStr for ResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Failure" => Ok(Self::Failure),
                "Success" => Ok(Self::Success),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ResponseStatus {
        fn default() -> Self {
            ResponseStatus::Success
        }
    }

    #[doc = "Fees that the states require e.g 'CO Retail Delivery Fee'"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RetailDeliveryFees\","]
    #[doc = "  \"description\": \"Fees that the states require e.g 'CO Retail Delivery Fee'\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"feeAmount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"description\": \"Total amount that you pay for this Fee\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"Name of the Fees require by State\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RetailDeliveryFees {
        #[serde(rename = "feeAmount", default, skip_serializing_if = "Option::is_none")]
        pub fee_amount: Option<f64>,
        #[doc = "Name of the Fees require by State"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&RetailDeliveryFees> for RetailDeliveryFees {
        fn from(value: &RetailDeliveryFees) -> Self {
            value.clone()
        }
    }

    #[doc = "RuleComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RuleComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceDescription\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"complianceDetails\": {"]
    #[doc = "      \"title\": \"ComplianceDetailResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ComplianceDetailResponse\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"customerAggregateVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/CustomerAggregateVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"perBottleVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/PerBottleVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"perShipmentVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/PerShipmentVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"ruleDescription\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ruleType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"supplier\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RuleComplianceResponse {
        #[serde(
            rename = "complianceDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_description: Option<String>,
        #[serde(
            rename = "complianceDetails",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_details: Option<Vec<ComplianceDetailResponse>>,
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(
            rename = "customerAggregateVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_aggregate_volume_limit_detail: Option<CustomerAggregateVolumeLimitDetail>,
        #[serde(
            rename = "licenseRelationship",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub license_relationship: Option<String>,
        #[serde(
            rename = "perBottleVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_bottle_volume_limit_detail: Option<PerBottleVolumeLimitDetail>,
        #[serde(
            rename = "perShipmentVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_shipment_volume_limit_detail: Option<PerShipmentVolumeLimitDetail>,
        #[serde(
            rename = "ruleDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rule_description: Option<String>,
        #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
        pub rule_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub supplier: Option<String>,
    }

    impl From<&RuleComplianceResponse> for RuleComplianceResponse {
        fn from(value: &RuleComplianceResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "RuleComplianceStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RuleComplianceStatus\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Compliant\","]
    #[doc = "    \"NotCompliant\","]
    #[doc = "    \"Bypassed\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleComplianceStatus {
        Compliant,
        NotCompliant,
        Bypassed,
    }

    impl From<&RuleComplianceStatus> for RuleComplianceStatus {
        fn from(value: &RuleComplianceStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for RuleComplianceStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Compliant => "Compliant".to_string(),
                Self::NotCompliant => "NotCompliant".to_string(),
                Self::Bypassed => "Bypassed".to_string(),
            }
        }
    }

    impl std::str::FromStr for RuleComplianceStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Compliant" => Ok(Self::Compliant),
                "NotCompliant" => Ok(Self::NotCompliant),
                "Bypassed" => Ok(Self::Bypassed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "SalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"billTo\","]
    #[doc = "    \"customerKey\","]
    #[doc = "    \"orderType\","]
    #[doc = "    \"purchaseDate\","]
    #[doc = "    \"salesOrderKey\","]
    #[doc = "    \"shipments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Payment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Shipment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrder {
        #[serde(rename = "billTo")]
        pub bill_to: Address,
        #[serde(rename = "customerKey")]
        pub customer_key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<SalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<String>,
        #[serde(rename = "orderType")]
        pub order_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<Payment>>,
        #[serde(rename = "purchaseDate")]
        pub purchase_date: chrono::DateTime<chrono::offset::Utc>,
        #[serde(rename = "salesOrderKey")]
        pub sales_order_key: String,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        pub shipments: Vec<Shipment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl From<&SalesOrder> for SalesOrder {
        fn from(value: &SalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderAddressValidationResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderAddressValidationResponse\","]
    #[doc = "  \"description\": \"SalesOrderAddressValidationResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SalesOrderAddressValidationResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderAddressValidationResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<Vec<SalesOrderAddressValidationResponseError>>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::sales_order_address_validation_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::sales_order_address_validation_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&SalesOrderAddressValidationResponse> for SalesOrderAddressValidationResponse {
        fn from(value: &SalesOrderAddressValidationResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderAddressValidationResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderAddressValidationResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"data\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ErrorData\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"SalesOrder\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderAddressValidationResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<ErrorData>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::sales_order_address_validation_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(default = "defaults::sales_order_address_validation_response_error_target")]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::sales_order_address_validation_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&SalesOrderAddressValidationResponseError> for SalesOrderAddressValidationResponseError {
        fn from(value: &SalesOrderAddressValidationResponseError) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderBase"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderBase\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"salesOrder\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"persistOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderBase {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<AddressOption>,
        #[serde(
            rename = "persistOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub persist_option: Option<String>,
        #[serde(rename = "salesOrder")]
        pub sales_order: SalesOrder,
    }

    impl From<&SalesOrderBase> for SalesOrderBase {
        fn from(value: &SalesOrderBase) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderComplianceTaxResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderComplianceTaxResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRates\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SalesTaxRates\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentComplianceResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentComplianceResponse\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderComplianceTaxResponse {
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(
            rename = "salesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rates: Option<SalesTaxRates>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments: Option<Vec<ShipmentComplianceResponse>>,
    }

    impl From<&SalesOrderComplianceTaxResponse> for SalesOrderComplianceTaxResponse {
        fn from(value: &SalesOrderComplianceTaxResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&SalesOrderDiscount> for SalesOrderDiscount {
        fn from(value: &SalesOrderDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SalesOrderNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderNotFoundResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<Vec<SalesOrderNotFoundResponseError>>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::sales_order_not_found_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::sales_order_not_found_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&SalesOrderNotFoundResponse> for SalesOrderNotFoundResponse {
        fn from(value: &SalesOrderNotFoundResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"SalesOrder\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::sales_order_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(default = "defaults::sales_order_not_found_response_error_target")]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::sales_order_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&SalesOrderNotFoundResponseError> for SalesOrderNotFoundResponseError {
        fn from(value: &SalesOrderNotFoundResponseError) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"FulfillmentType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Club\","]
    #[doc = "        \"Daily\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"OrderTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Club\","]
    #[doc = "        \"Fax\","]
    #[doc = "        \"InPerson\","]
    #[doc = "        \"Internet\","]
    #[doc = "        \"Mail\","]
    #[doc = "        \"Phone\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/PaymentOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesOrderOutput {
        #[serde(rename = "billTo", default, skip_serializing_if = "Option::is_none")]
        pub bill_to: Option<Address>,
        #[serde(
            rename = "customerKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<SalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<FulfillmentType>,
        #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
        pub order_type: Option<OrderTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<PaymentOutput>>,
        #[serde(
            rename = "purchaseDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub purchase_date: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "salesOrderKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order_key: Option<String>,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments: Option<Vec<ShipmentOutput>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl From<&SalesOrderOutput> for SalesOrderOutput {
        fn from(value: &SalesOrderOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesTaxRates"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesTaxRates\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"crvSupplementalFees\": {"]
    #[doc = "      \"title\": \"IList`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/RetailDeliveryFees\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"recommendedSalesTaxDue\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"retailDeliveryFees\": {"]
    #[doc = "      \"title\": \"IList`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/RetailDeliveryFees\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentSalesTaxRates\": {"]
    #[doc = "      \"title\": \"ShipmentSalesTaxRateCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentSalesTaxRate\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SalesTaxRates {
        #[serde(
            rename = "crvSupplementalFees",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub crv_supplemental_fees: Option<Vec<RetailDeliveryFees>>,
        #[serde(
            rename = "recommendedSalesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub recommended_sales_tax_due: Option<f64>,
        #[serde(
            rename = "retailDeliveryFees",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub retail_delivery_fees: Option<Vec<RetailDeliveryFees>>,
        #[serde(
            rename = "shipmentSalesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_sales_tax_rates: Option<Vec<ShipmentSalesTaxRate>>,
    }

    impl From<&SalesTaxRates> for SalesTaxRates {
        fn from(value: &SalesTaxRates) -> Self {
            value.clone()
        }
    }

    #[doc = "Shipment"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Shipment\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"licenseRelationship\","]
    #[doc = "    \"shipDate\","]
    #[doc = "    \"shipTo\","]
    #[doc = "    \"shipmentItems\","]
    #[doc = "    \"shipmentStatus\","]
    #[doc = "    \"shippingService\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentAccount\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionReason\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentHouse\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"giftNote\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"handling\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"packages\": {"]
    #[doc = "      \"title\": \"PackageCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Package\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"shipTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Address\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentItem\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipmentStatus\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"shipping\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shippingService\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"specialInstructions\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Shipment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipmentDiscount>>,
        #[serde(
            rename = "fulfillmentAccount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_account: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_reason: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_type: Option<String>,
        #[serde(
            rename = "fulfillmentHouse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_house: Option<String>,
        #[serde(rename = "giftNote", default, skip_serializing_if = "Option::is_none")]
        pub gift_note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub handling: Option<f64>,
        #[serde(rename = "licenseRelationship")]
        pub license_relationship: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packages: Option<Vec<Package>>,
        #[serde(rename = "shipDate")]
        pub ship_date: chrono::DateTime<chrono::offset::Utc>,
        #[serde(rename = "shipTo")]
        pub ship_to: Address,
        #[serde(rename = "shipmentItems")]
        pub shipment_items: Vec<ShipmentItem>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(rename = "shipmentStatus")]
        pub shipment_status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipping: Option<f64>,
        #[serde(rename = "shippingService")]
        pub shipping_service: String,
        #[serde(
            rename = "specialInstructions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub special_instructions: Option<String>,
    }

    impl From<&Shipment> for Shipment {
        fn from(value: &Shipment) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"rules\": {"]
    #[doc = "      \"title\": \"RuleComplianceResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/RuleComplianceResponse\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentComplianceResponse {
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rules: Option<Vec<RuleComplianceResponse>>,
    }

    impl From<&ShipmentComplianceResponse> for ShipmentComplianceResponse {
        fn from(value: &ShipmentComplianceResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipmentDiscount> for ShipmentDiscount {
        fn from(value: &ShipmentDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentItem"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItem\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"productKey\","]
    #[doc = "    \"productQuantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"citb\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentItemDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentItemDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productQuantity\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productUnitPrice\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentItem {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub citb: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipmentItemDiscount>>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(rename = "productQuantity")]
        pub product_quantity: f64,
        #[serde(
            rename = "productUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_unit_price: Option<f64>,
    }

    impl From<&ShipmentItem> for ShipmentItem {
        fn from(value: &ShipmentItem) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentItemDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItemDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentItemDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipmentItemDiscount> for ShipmentItemDiscount {
        fn from(value: &ShipmentItemDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentItemForTax"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItemForTax\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"productKey\","]
    #[doc = "    \"productQuantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productQuantity\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productUnitPrice\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentItemForTax {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(rename = "productQuantity")]
        pub product_quantity: f64,
        #[serde(
            rename = "productUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_unit_price: Option<f64>,
    }

    impl From<&ShipmentItemForTax> for ShipmentItemForTax {
        fn from(value: &ShipmentItemForTax) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentAccount\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionReason\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionType\": {"]
    #[doc = "      \"title\": \"FulfillmentExceptionTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"None\","]
    #[doc = "        \"Inventory\","]
    #[doc = "        \"NonCompliant\","]
    #[doc = "        \"Other\","]
    #[doc = "        \"Updated\","]
    #[doc = "        \"Setup\","]
    #[doc = "        \"Temperature\","]
    #[doc = "        \"Voided\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentHouse\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"giftNote\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"handling\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"LicenseRelationshipTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Default\","]
    #[doc = "        \"Pickup\","]
    #[doc = "        \"RetailerToConsumer\","]
    #[doc = "        \"RetailerToThreeTier\","]
    #[doc = "        \"SupplierToConsumer\","]
    #[doc = "        \"SupplierToThreeTier\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"packages\": {"]
    #[doc = "      \"title\": \"PackageOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/PackageOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"shipTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Address\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipmentItem\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipmentStatus\": {"]
    #[doc = "      \"title\": \"ShipmentStatusTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Delivered\","]
    #[doc = "        \"InProcess\","]
    #[doc = "        \"Shipped\","]
    #[doc = "        \"Voided\","]
    #[doc = "        \"SentToFulfillment\","]
    #[doc = "        \"PaymentAccepted\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipping\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shippingService\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"specialInstructions\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipmentDiscount>>,
        #[serde(
            rename = "fulfillmentAccount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_account: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_reason: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_type: Option<FulfillmentExceptionTypes>,
        #[serde(
            rename = "fulfillmentHouse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_house: Option<String>,
        #[serde(rename = "giftNote", default, skip_serializing_if = "Option::is_none")]
        pub gift_note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub handling: Option<f64>,
        #[serde(
            rename = "licenseRelationship",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub license_relationship: Option<LicenseRelationshipTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packages: Option<Vec<PackageOutput>>,
        #[serde(rename = "shipDate", default, skip_serializing_if = "Option::is_none")]
        pub ship_date: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "shipTo", default, skip_serializing_if = "Option::is_none")]
        pub ship_to: Option<Address>,
        #[serde(
            rename = "shipmentItems",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_items: Option<Vec<ShipmentItem>>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(
            rename = "shipmentStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_status: Option<ShipmentStatusTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipping: Option<f64>,
        #[serde(
            rename = "shippingService",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_service: Option<String>,
        #[serde(
            rename = "specialInstructions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub special_instructions: Option<String>,
    }

    impl From<&ShipmentOutput> for ShipmentOutput {
        fn from(value: &ShipmentOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"freightSalesTaxRate\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/FreightSalesTaxRate\""]
    #[doc = "    },"]
    #[doc = "    \"productSalesTaxRates\": {"]
    #[doc = "      \"title\": \"ProductSalesTaxRateCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ProductSalesTaxRate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipmentSalesTaxRate {
        #[serde(
            rename = "freightSalesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub freight_sales_tax_rate: Option<FreightSalesTaxRate>,
        #[serde(
            rename = "productSalesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_sales_tax_rates: Option<Vec<ProductSalesTaxRate>>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
    }

    impl From<&ShipmentSalesTaxRate> for ShipmentSalesTaxRate {
        fn from(value: &ShipmentSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentStatusTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentStatusTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Delivered\","]
    #[doc = "    \"InProcess\","]
    #[doc = "    \"Shipped\","]
    #[doc = "    \"Voided\","]
    #[doc = "    \"SentToFulfillment\","]
    #[doc = "    \"PaymentAccepted\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ShipmentStatusTypes {
        Delivered,
        InProcess,
        Shipped,
        Voided,
        SentToFulfillment,
        PaymentAccepted,
    }

    impl From<&ShipmentStatusTypes> for ShipmentStatusTypes {
        fn from(value: &ShipmentStatusTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for ShipmentStatusTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Delivered => "Delivered".to_string(),
                Self::InProcess => "InProcess".to_string(),
                Self::Shipped => "Shipped".to_string(),
                Self::Voided => "Voided".to_string(),
                Self::SentToFulfillment => "SentToFulfillment".to_string(),
                Self::PaymentAccepted => "PaymentAccepted".to_string(),
            }
        }
    }

    impl std::str::FromStr for ShipmentStatusTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Delivered" => Ok(Self::Delivered),
                "InProcess" => Ok(Self::InProcess),
                "Shipped" => Ok(Self::Shipped),
                "Voided" => Ok(Self::Voided),
                "SentToFulfillment" => Ok(Self::SentToFulfillment),
                "PaymentAccepted" => Ok(Self::PaymentAccepted),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "StatusCode"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"StatusCode\","]
    #[doc = "  \"default\": 200,"]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    200,"]
    #[doc = "    400,"]
    #[doc = "    401,"]
    #[doc = "    403,"]
    #[doc = "    404,"]
    #[doc = "    429,"]
    #[doc = "    500"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct StatusCode(i64);
    impl ::std::ops::Deref for StatusCode {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<StatusCode> for i64 {
        fn from(value: StatusCode) -> Self {
            value.0
        }
    }

    impl From<&StatusCode> for StatusCode {
        fn from(value: &StatusCode) -> Self {
            value.clone()
        }
    }

    impl Default for StatusCode {
        fn default() -> Self {
            StatusCode(200_i64)
        }
    }

    impl std::convert::TryFrom<i64> for StatusCode {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
            if ![
                200_i64, 400_i64, 401_i64, 403_i64, 404_i64, 429_i64, 500_i64,
            ]
            .contains(&value)
            {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    #[doc = "Tag"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Tag\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Tag {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&Tag> for Tag {
        fn from(value: &Tag) -> Self {
            value.clone()
        }
    }

    #[doc = "TaxRateAddressInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateAddressInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"zip1\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct TaxRateAddressInput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        pub zip1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&TaxRateAddressInput> for TaxRateAddressInput {
        fn from(value: &TaxRateAddressInput) -> Self {
            value.clone()
        }
    }

    #[doc = "TaxRateAddressOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateAddressOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressStatus\": {"]
    #[doc = "      \"title\": \"AddressStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"AddressOutOfRange\","]
    #[doc = "        \"AddressSuggested\","]
    #[doc = "        \"ComponentMismatch\","]
    #[doc = "        \"MultipleMatches\","]
    #[doc = "        \"NonDeliverableAddress\","]
    #[doc = "        \"NoStreetData\","]
    #[doc = "        \"UnknownStreet\","]
    #[doc = "        \"Validated\","]
    #[doc = "        \"ZipCodeDoesNotExist\","]
    #[doc = "        \"ZipCodeDoesNotMatchCityState\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct TaxRateAddressOutput {
        #[serde(
            rename = "addressStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_status: Option<AddressStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&TaxRateAddressOutput> for TaxRateAddressOutput {
        fn from(value: &TaxRateAddressOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "TimeFrameType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TimeFrameType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Calendar\","]
    #[doc = "    \"Rolling\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TimeFrameType {
        Calendar,
        Rolling,
    }

    impl From<&TimeFrameType> for TimeFrameType {
        fn from(value: &TimeFrameType) -> Self {
            value.clone()
        }
    }

    impl ToString for TimeFrameType {
        fn to_string(&self) -> String {
            match *self {
                Self::Calendar => "Calendar".to_string(),
                Self::Rolling => "Rolling".to_string(),
            }
        }
    }

    impl std::str::FromStr for TimeFrameType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Calendar" => Ok(Self::Calendar),
                "Rolling" => Ok(Self::Rolling),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "TimeFrameUnit"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TimeFrameUnit\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Minutes\","]
    #[doc = "    \"Hours\","]
    #[doc = "    \"Days\","]
    #[doc = "    \"Weeks\","]
    #[doc = "    \"Months\","]
    #[doc = "    \"Years\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TimeFrameUnit {
        Null,
        Minutes,
        Hours,
        Days,
        Weeks,
        Months,
        Years,
    }

    impl From<&TimeFrameUnit> for TimeFrameUnit {
        fn from(value: &TimeFrameUnit) -> Self {
            value.clone()
        }
    }

    impl ToString for TimeFrameUnit {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Minutes => "Minutes".to_string(),
                Self::Hours => "Hours".to_string(),
                Self::Days => "Days".to_string(),
                Self::Weeks => "Weeks".to_string(),
                Self::Months => "Months".to_string(),
                Self::Years => "Years".to_string(),
            }
        }
    }

    impl std::str::FromStr for TimeFrameUnit {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Minutes" => Ok(Self::Minutes),
                "Hours" => Ok(Self::Hours),
                "Days" => Ok(Self::Days),
                "Weeks" => Ok(Self::Weeks),
                "Months" => Ok(Self::Months),
                "Years" => Ok(Self::Years),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "VolumeUnit"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"VolumeUnit\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Bottle\","]
    #[doc = "    \"Case\","]
    #[doc = "    \"Gallon\","]
    #[doc = "    \"Liter\","]
    #[doc = "    \"Milliliter\","]
    #[doc = "    \"Ounce\","]
    #[doc = "    \"Quart\","]
    #[doc = "    \"Barrel\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum VolumeUnit {
        Null,
        Bottle,
        Case,
        Gallon,
        Liter,
        Milliliter,
        Ounce,
        Quart,
        Barrel,
    }

    impl From<&VolumeUnit> for VolumeUnit {
        fn from(value: &VolumeUnit) -> Self {
            value.clone()
        }
    }

    impl ToString for VolumeUnit {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Bottle => "Bottle".to_string(),
                Self::Case => "Case".to_string(),
                Self::Gallon => "Gallon".to_string(),
                Self::Liter => "Liter".to_string(),
                Self::Milliliter => "Milliliter".to_string(),
                Self::Ounce => "Ounce".to_string(),
                Self::Quart => "Quart".to_string(),
                Self::Barrel => "Barrel".to_string(),
            }
        }
    }

    impl std::str::FromStr for VolumeUnit {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Bottle" => Ok(Self::Bottle),
                "Case" => Ok(Self::Case),
                "Gallon" => Ok(Self::Gallon),
                "Liter" => Ok(Self::Liter),
                "Milliliter" => Ok(Self::Milliliter),
                "Ounce" => Ok(Self::Ounce),
                "Quart" => Ok(Self::Quart),
                "Barrel" => Ok(Self::Barrel),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = r" Generation of default values for serde."]
    pub mod defaults {
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: std::convert::TryFrom<u64>,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }

        pub(super) fn check_compliance_response_response_status() -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn check_compliance_response_status_code() -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn get_sales_order_success_response_response_status() -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn get_sales_order_success_response_status_code() -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn quote_tax_sales_order_response_response_status() -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn quote_tax_sales_order_response_status_code() -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn sales_order_address_validation_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn sales_order_address_validation_response_status_code() -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn sales_order_address_validation_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn sales_order_address_validation_response_error_target() -> super::ErrorTarget {
            super::ErrorTarget::SalesOrder
        }

        pub(super) fn sales_order_address_validation_response_error_type() -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn sales_order_not_found_response_response_status() -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn sales_order_not_found_response_status_code() -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn sales_order_not_found_response_error_status_code() -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn sales_order_not_found_response_error_target() -> super::ErrorTarget {
            super::ErrorTarget::SalesOrder
        }

        pub(super) fn sales_order_not_found_response_error_type() -> super::ErrorType {
            super::ErrorType::Validation
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone {
        SalesOrderNotFoundResponse(SalesOrderNotFoundResponse),
        SalesOrderAddressValidationResponse(SalesOrderAddressValidationResponse),
        None,
    }

    impl From<SalesOrderNotFoundResponse>
        for SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone
    {
        fn from(value: SalesOrderNotFoundResponse) -> Self {
            SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone :: SalesOrderNotFoundResponse (value)
        }
    }

    impl From<SalesOrderAddressValidationResponse>
        for SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone
    {
        fn from(value: SalesOrderAddressValidationResponse) -> Self {
            SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone :: SalesOrderAddressValidationResponse (value)
        }
    }

    impl From<()> for SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone {
        fn from(_: ()) -> Self {
            SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone::None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum SalesOrderNotFoundResponseOrNone {
        SalesOrderNotFoundResponse(SalesOrderNotFoundResponse),
        None,
    }

    impl From<SalesOrderNotFoundResponse> for SalesOrderNotFoundResponseOrNone {
        fn from(value: SalesOrderNotFoundResponse) -> Self {
            SalesOrderNotFoundResponseOrNone::SalesOrderNotFoundResponse(value)
        }
    }

    impl From<()> for SalesOrderNotFoundResponseOrNone {
        fn from(_: ()) -> Self {
            SalesOrderNotFoundResponseOrNone::None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum SalesOrderAddressValidationResponseOrNone {
        SalesOrderAddressValidationResponse(SalesOrderAddressValidationResponse),
        None,
    }

    impl From<SalesOrderAddressValidationResponse> for SalesOrderAddressValidationResponseOrNone {
        fn from(value: SalesOrderAddressValidationResponse) -> Self {
            SalesOrderAddressValidationResponseOrNone::SalesOrderAddressValidationResponse(value)
        }
    }

    impl From<()> for SalesOrderAddressValidationResponseOrNone {
        fn from(_: ()) -> Self {
            SalesOrderAddressValidationResponseOrNone::None
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for ShipCompliant Connect\n\nShipCompliant REST API supporting eCommerce and Point of Sale workflows. <br /> * required fields.\n\nhttps://www.sovos.com/shipcompliant/terms-and-conditions/\n\nVersion: v2.0"]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    #[doc = r" Create a new client."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    #[doc = r" Construct a new client with an existing `reqwest::Client`,"]
    #[doc = r" allowing more control over its configuration."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    #[doc = r" Get the base URL to which requests are made."]
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    #[doc = r" Get the internal `reqwest::Client` used to make requests."]
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    #[doc = r" Get the version of this API."]
    #[doc = r""]
    #[doc = r" This string is pulled directly from the source OpenAPI"]
    #[doc = r" document and may be in any format the API selects."]
    pub fn api_version(&self) -> &'static str {
        "v2.0"
    }
}

#[allow(clippy::all)]
impl Client {
    #[doc = "Retrieves the specified sales order\n\nSample request:\r\n            \r\n    GET /api/v2.0/salesOrders/ORDER123\n\nSends a `GET` request to `/api/v2.0/salesOrders/{salesOrderKey}`\n\nArguments:\n- `sales_order_key`: The sales order number\n"]
    pub async fn get_api_v2_0_sales_orders_sales_order_key<'a>(
        &'a self,
        sales_order_key: &'a str,
    ) -> Result<
        ResponseValue<types::GetSalesOrderSuccessResponse>,
        Error<types::SalesOrderNotFoundResponseOrNone>,
    > {
        let url = format!(
            "{}/api/v2.0/salesOrders/{}",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order that will be committed at a later time\n\nSample request:\r\n            \r\n\tPOST /api/v2.0/salesOrders/check-compliance\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  },\r\n\t  \"PersistOption\": \"Null\"\r\n\t}\n\nSends a `POST` request to `/api/v2.0/salesOrders/check-compliance`\n\nArguments:\n- `body`: The sales order to check compliance\n"]
    pub async fn post_api_v2_0_sales_orders_check_compliance<'a>(
        &'a self,
        body: &'a types::SalesOrderBase,
    ) -> Result<
        ResponseValue<types::CheckComplianceResponse>,
        Error<types::SalesOrderAddressValidationResponseOrNone>,
    > {
        let url = format!("{}/api/v2.0/salesOrders/check-compliance", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order and saves the order to the system\n\nSample request:\r\n            \r\n\tPOST /api/v2.0/salesOrders/check-commit\r\n\t{\r\n\t  \"CommitOption\": \"AllShipments\",\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  },\r\n\t  \"PersistOption\": \"Null\"\r\n\t}\n\nSends a `POST` request to `/api/v2.0/salesOrders/check-commit`\n\nArguments:\n- `body`: The sales order to check compliance\n"]
    pub async fn post_api_v2_0_sales_orders_check_commit<'a>(
        &'a self,
        body: &'a types::CheckAndCommitSalesOrder,
    ) -> Result<
        ResponseValue<types::CheckComplianceResponse>,
        Error<types::SalesOrderNotFoundResponseOrSalesOrderAddressValidationResponseOrNone>,
    > {
        let url = format!("{}/api/v2.0/salesOrders/check-commit", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order without saving results for future use\n\nSample request:\r\n            \r\n\tPOST /api/v2.0/salesOrders/quote\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  }\r\n\t}\n\nSends a `POST` request to `/api/v2.0/salesOrders/quote`\n\nArguments:\n- `body`: The sales order to Quote\n"]
    pub async fn post_api_v2_0_sales_orders_quote<'a>(
        &'a self,
        body: &'a types::QuoteSalesOrderInput,
    ) -> Result<
        ResponseValue<types::CheckComplianceResponse>,
        Error<types::SalesOrderAddressValidationResponseOrNone>,
    > {
        let url = format!("{}/api/v2.0/salesOrders/quote", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the tax due for a sales order without saving results for future use\n\nSample request:\r\n            \r\n\tPOST /api/v2.0/salesOrders/quote/sales-tax\r\n\t{\r\n\t  \"ShipToAddress\": {\r\n\t\t\"City\":  \"Boulder\",\r\n\t    \"State\": \"CO\",\r\n\t    \"Street1\": \"2465 Central Ave\",\r\n\t    \"Street2\": \"Ste 110\",\r\n\t    \"Zip1\": \"80301\",\r\n\t    \"Zip2\": \"5728\"\r\n\t  },\r\n\t  \"EffectiveDate\": \"2020-11-01T00:00:00Z\",\r\n\t  \"TaxSaleType\": \"onsite\",\r\n\t  \"ShippingAndHandlingCollected\": 0.00,\r\n\t  \"OrderItems\": [\r\n\t    {\r\n\t      \"BrandKey\": \"Brand123\",\r\n\t      \"ProductKey\": \"Product123\",\r\n\t      \"ProductQuantity\": 2,\r\n\t      \"ProductUnitPrice\": 19.99\r\n\t    }\r\n\t  ]\r\n\t}\n\nSends a `POST` request to `/api/v2.0/salesOrders/quote/sales-tax`\n\nArguments:\n- `body`: The sales order to Quote the tax\n"]
    pub async fn post_api_v2_0_sales_orders_quote_sales_tax<'a>(
        &'a self,
        body: &'a types::QuoteTaxForSalesOrder,
    ) -> Result<
        ResponseValue<types::QuoteTaxSalesOrderResponse>,
        Error<types::SalesOrderAddressValidationResponseOrNone>,
    > {
        let url = format!("{}/api/v2.0/salesOrders/quote/sales-tax", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

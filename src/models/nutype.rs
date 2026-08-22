use crate::models::error::WayclipError;
use nutype::nutype;

// horrible looking macros, but what they do is basically
// 1. Create a new nutype struct
// 2. Assign rules, like regex, length, etc..
// 3. create utopia traits (only if feature = "openapi" is enabled)
macro_rules! validated_string {
    ($type:ident, as: $name:expr, max: $max_len:expr, example: $example:expr) => {
        #[nutype(
            sanitize(trim),
            validate(len_char_max = $max_len),
            derive(Debug, PartialEq, Eq, Clone, TryFrom, Serialize, Deserialize)
        )]
        pub struct $type(String);

        ::paste::paste! {
            impl From<[<$type Error>]> for WayclipError {
                fn from(err: [<$type Error>]) -> Self {
                    WayclipError::Validation(std::borrow::Cow::Owned(err.to_string()))
                }
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::ToSchema for $type {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::PartialSchema for $type {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String)
                    .max_length(Some($max_len))
                    .examples([serde_json::json!($example)])
                    .into()
            }
        }
    };

    ($type:ident, as: $name:expr, min: $min:expr, max: $max:expr, regex: $regex:expr, example: $example:expr) => {
        #[nutype(
            sanitize(trim),
            validate(not_empty, len_char_min = $min, len_char_max = $max, regex = $regex),
            derive(Debug, PartialEq, Eq, Clone, TryFrom, Serialize, Deserialize)
        )]
        pub struct $type(String);

        ::paste::paste! {
            impl From<[<$type Error>]> for WayclipError {
                fn from(err: [<$type Error>]) -> Self {
                    WayclipError::Validation(std::borrow::Cow::Owned(err.to_string()))
                }
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::ToSchema for $type {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::PartialSchema for $type {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String)
                    .min_length(Some($min))
                    .max_length(Some($max))
                    .pattern(Some($regex))
                    .examples([serde_json::json!($example)])
                    .into()
            }
        }
    };

    ($type:ident, as: $name:expr, sanitize: [$($sanitizers:tt),*], regex: $regex:expr, example: $example:expr) => {
        #[nutype(
            sanitize($($sanitizers),*),
            validate(not_empty, regex = $regex),
            derive(Debug, PartialEq, Eq, Clone, TryFrom, Serialize, Deserialize)
        )]
        pub struct $type(String);

        ::paste::paste! {
            impl From<[<$type Error>]> for WayclipError {
                fn from(err: [<$type Error>]) -> Self {
                    WayclipError::Validation(std::borrow::Cow::Owned(err.to_string()))
                }
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::ToSchema for $type {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::PartialSchema for $type {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String)
                    .pattern(Some($regex))
                    .examples([serde_json::json!($example)])
                    .into()
            }
        }
    };
}

macro_rules! validated_int {
    ($type:ident, as: $name:expr, min: $min:expr, max: $max:expr, example: $example:expr) => {
        #[nutype(
            validate(greater_or_equal = $min, less_or_equal = $max),
            derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFrom, Serialize, Deserialize)
        )]
        pub struct $type(i32);

        ::paste::paste! {
            impl From<[<$type Error>]> for WayclipError {
                fn from(err: [<$type Error>]) -> Self {
                    WayclipError::Validation(std::borrow::Cow::Owned(err.to_string()))
                }
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::ToSchema for $type {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::PartialSchema for $type {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .minimum(Some($min))
                    .maximum(Some($max))
                    .examples([serde_json::json!($example)])
                    .into()
            }
        }
    };
    ($type:ident, as: $name:expr, min: $min:expr, example: $example:expr) => {
        #[nutype(
            validate(greater_or_equal = $min),
            derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFrom, Serialize, Deserialize)
        )]
        pub struct $type(i32);

        ::paste::paste! {
            impl From<[<$type Error>]> for WayclipError {
                fn from(err: [<$type Error>]) -> Self {
                    WayclipError::Validation(std::borrow::Cow::Owned(err.to_string()))
                }
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::ToSchema for $type {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed($name)
            }
        }

        #[cfg(feature = "openapi")]
        impl utoipa::PartialSchema for $type {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .minimum(Some($min))
                    .examples([serde_json::json!($example)])
                    .into()
            }
        }
    };
}

validated_string!(LocaleSanitised, as: "Locale", min: 2, max: 64, regex: r"^(?i)[a-z]{2,3}(?:-[a-z0-9]{2,8})*$", example: "en-US");
validated_string!(UsernameSanitised, as: "Username", min: 3, max: 32, regex: r"^[a-zA-Z0-9_-]+$", example: "cool_user");
validated_string!(RoleNameSanitised, as: "RoleName", min: 3, max: 100, regex: r"^[a-zA-Z0-9_-]+$", example: "admin");
validated_string!(LimitNameSanitised, as: "LimitName", min: 3, max: 100, regex: r"^[a-zA-Z0-9_-]+$", example: "standard_tier");
validated_string!(ClipNameSanitised, as: "ClipName", min: 3, max: 100, regex: r"^[a-zA-Z0-9_ -]+$", example: "pentakill_clip");

validated_string!(UsersAboutMeSanitised, as: "UsersAboutMe", max: 384, example: "This is an about me section.");
validated_string!(LimitDescriptionSanitised, as: "LimitDescription", max: 200, example: "This is a description of the limit.");
validated_string!(ClipCommentContentSanitised, as: "ClipCommentContent", max: 200, example: "This is the content of a clip comment.");
validated_string!(AppealMessageSanitised, as: "AppealMessage", max: 300, example: "This is an appeal message.");
validated_string!(BanDescriptionSanitised, as: "BanDescription", max: 200, example: "This is a ban description.");
validated_string!(ReportDescriptionSanitised, as: "ReportDescription", max: 200, example: "This is a report description.");
validated_string!(BanLiftDescriptionSanitised, as: "BanLiftDescription", max: 200, example: "This is a ban lift description.");

validated_string!(EmailSanitised, as: "Email", min: 5, max: 254, regex: r"(?i)^[a-z0-9_'+-]+(?:\.[a-z0-9_'+-]+)*@(?:[a-z0-9][a-z0-9-]*\.)+[a-z]{2,}$", example: "user@example.com");
validated_string!(TagNameSanitised, as: "TagName", min: 1, max: 19, regex: r"^[\p{L}\p{N}_-]+$", example: "gaming");
validated_string!(HexSanitised, as: "Hex", min: 0, max: 6, regex: r"^[0-9a-f]+$", example: "a3f89e");

validated_string!(ResolutionSanitised, as: "Resolution", sanitize: [trim, lowercase], regex: r"^[1-9][0-9]*x[1-9][0-9]*$", example: "1920x1080");
validated_string!(DeviceUserCodeSanitised, as: "DeviceUserCode", sanitize: [trim, uppercase], regex: r"^[34679A-HJ-NP-Z]{8}$", example: "W9KTRVXY");
validated_string!(PermissionScopeSanitised, as: "PermissionScope", sanitize: [trim, lowercase], regex: r"^(?:\*|[a-z0-9_-]+:(?:\*|[a-z0-9_-]+))$", example: "user:read");

validated_int!(LimitMbSanitised, as: "LimitMb", min: 1, max: 1000000, example: 1024);
validated_int!(ClipDurationSanitised, as: "ClipDuration", min: 1, example: 120);
validated_int!(BitrateKbpsSanitised, as: "BitrateKbps", min: 1, example: 4500);
validated_int!(FpsSanitised, as: "Fps", min: 1, max: 1000, example: 60);

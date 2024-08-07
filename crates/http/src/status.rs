/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// status

// ----------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    Informational = 1,
    Success = 2,
    Redirection = 3,
    ClientError = 4,
    ServerError = 5,
}

// ----------------------------------------------------------------

/// @see https://datatracker.ietf.org/doc/html/rfc7231#section-6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    // Informational 1xx
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // Success 2xx
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    // Redirection 3xx
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // Client Error 4xx
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // Server Error 5xx
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HttpStatus {
    pub fn value(&self) -> u16 {
        *self as u16
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Processing => "Processing",
            HttpStatus::EarlyHints => "Early Hints",
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatus::NoContent => "No Content",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "Partial Content",
            HttpStatus::MultipleChoices => "Multiple Choices",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::UriTooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::ImATeapot => "I'm a teapot",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }

    // ----------------------------------------------------------------

    pub fn series(&self) -> Result<Series, &'static str> {
        match self.value() {
            100..=199 => Ok(Series::Informational),
            200..=299 => Ok(Series::Success),
            300..=399 => Ok(Series::Redirection),
            400..=499 => Ok(Series::ClientError),
            500..=599 => Ok(Series::ServerError),
            _ => Err("Invalid status code"),
        }
    }

    // ----------------------------------------------------------------

    pub fn is_informational(&self) -> bool {
        matches!(self.series(), Ok(Series::Informational))
    }

    pub fn is_success(&self) -> bool {
        matches!(self.series(), Ok(Series::Success))
    }

    pub fn is_redirection(&self) -> bool {
        matches!(self.series(), Ok(Series::Redirection))
    }

    pub fn is_4xx_client_error(&self) -> bool {
        matches!(self.series(), Ok(Series::ClientError))
    }

    pub fn is_5xx_server_error(&self) -> bool {
        matches!(self.series(), Ok(Series::ServerError))
    }

    pub fn is_error(&self) -> bool {
        self.is_4xx_client_error() || self.is_5xx_server_error()
    }

    // ----------------------------------------------------------------

    pub fn value_of(status_code: u16) -> Result<HttpStatus, String> {
        match Self::resolve(status_code) {
            Some(status) => Ok(status),
            None => {
                let message = format!("No HttpStatus for code {}", status_code);
                Err(message)
            }
        }
    }

    pub fn resolve(status_code: u16) -> Option<HttpStatus> {
        match status_code {
            100 => Some(HttpStatus::Continue),
            101 => Some(HttpStatus::SwitchingProtocols),
            102 => Some(HttpStatus::Processing),
            103 => Some(HttpStatus::EarlyHints),
            200 => Some(HttpStatus::Ok),
            201 => Some(HttpStatus::Created),
            202 => Some(HttpStatus::Accepted),
            203 => Some(HttpStatus::NonAuthoritativeInformation),
            204 => Some(HttpStatus::NoContent),
            205 => Some(HttpStatus::ResetContent),
            206 => Some(HttpStatus::PartialContent),
            300 => Some(HttpStatus::MultipleChoices),
            301 => Some(HttpStatus::MovedPermanently),
            302 => Some(HttpStatus::Found),
            303 => Some(HttpStatus::SeeOther),
            304 => Some(HttpStatus::NotModified),
            305 => Some(HttpStatus::UseProxy),
            307 => Some(HttpStatus::TemporaryRedirect),
            308 => Some(HttpStatus::PermanentRedirect),
            400 => Some(HttpStatus::BadRequest),
            401 => Some(HttpStatus::Unauthorized),
            402 => Some(HttpStatus::PaymentRequired),
            403 => Some(HttpStatus::Forbidden),
            404 => Some(HttpStatus::NotFound),
            405 => Some(HttpStatus::MethodNotAllowed),
            406 => Some(HttpStatus::NotAcceptable),
            407 => Some(HttpStatus::ProxyAuthenticationRequired),
            408 => Some(HttpStatus::RequestTimeout),
            409 => Some(HttpStatus::Conflict),
            410 => Some(HttpStatus::Gone),
            411 => Some(HttpStatus::LengthRequired),
            412 => Some(HttpStatus::PreconditionFailed),
            413 => Some(HttpStatus::PayloadTooLarge),
            414 => Some(HttpStatus::UriTooLong),
            415 => Some(HttpStatus::UnsupportedMediaType),
            416 => Some(HttpStatus::RangeNotSatisfiable),
            417 => Some(HttpStatus::ExpectationFailed),
            418 => Some(HttpStatus::ImATeapot),
            421 => Some(HttpStatus::MisdirectedRequest),
            422 => Some(HttpStatus::UnprocessableEntity),
            423 => Some(HttpStatus::Locked),
            424 => Some(HttpStatus::FailedDependency),
            425 => Some(HttpStatus::TooEarly),
            426 => Some(HttpStatus::UpgradeRequired),
            428 => Some(HttpStatus::PreconditionRequired),
            429 => Some(HttpStatus::TooManyRequests),
            431 => Some(HttpStatus::RequestHeaderFieldsTooLarge),
            451 => Some(HttpStatus::UnavailableForLegalReasons),
            500 => Some(HttpStatus::InternalServerError),
            501 => Some(HttpStatus::NotImplemented),
            502 => Some(HttpStatus::BadGateway),
            503 => Some(HttpStatus::ServiceUnavailable),
            504 => Some(HttpStatus::GatewayTimeout),
            505 => Some(HttpStatus::HttpVersionNotSupported),
            506 => Some(HttpStatus::VariantAlsoNegotiates),
            507 => Some(HttpStatus::InsufficientStorage),
            508 => Some(HttpStatus::LoopDetected),
            510 => Some(HttpStatus::NotExtended),
            511 => Some(HttpStatus::NetworkAuthenticationRequired),
            _ => None,
        }
    }
}

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

// tests

// ----------------------------------------------------------------

use crate::status::{HttpStatus, Series};

// ----------------------------------------------------------------

#[test]
fn test_http_status() {
    let status = HttpStatus::Ok;

    assert_eq!(status.value(), 200);
    assert_eq!(status.reason_phrase(), "OK");
    assert_eq!(status.series(), Ok(Series::Success));
}

#[test]
fn test_http_status_reason_phrase() {
    assert_eq!(HttpStatus::Processing.reason_phrase(), "Processing");
    assert_eq!(HttpStatus::Ok.reason_phrase(), "OK");
    assert_eq!(HttpStatus::Found.reason_phrase(), "Found");
    assert_eq!(HttpStatus::BadRequest.reason_phrase(), "Bad Request");
    assert_eq!(
        HttpStatus::InternalServerError.reason_phrase(),
        "Internal Server Error"
    );
}

#[test]
fn test_http_status_series() {
    assert_eq!(HttpStatus::Processing.series(), Ok(Series::Informational));
    assert_eq!(HttpStatus::Ok.series(), Ok(Series::Success));
    assert_eq!(HttpStatus::Found.series(), Ok(Series::Redirection));
    assert_eq!(HttpStatus::BadRequest.series(), Ok(Series::ClientError));
    assert_eq!(
        HttpStatus::InternalServerError.series(),
        Ok(Series::ServerError)
    );
}

#[test]
fn test_http_status_is_xxx() {
    let status_ok = HttpStatus::Ok;
    assert!(status_ok.is_success());
    assert!(!status_ok.is_redirection());
    assert!(!status_ok.is_4xx_client_error());
    assert!(!status_ok.is_5xx_server_error());
    assert!(!status_ok.is_error());

    let status_bad_request = HttpStatus::BadRequest;
    assert!(!status_bad_request.is_success());
    assert!(!status_bad_request.is_redirection());
    assert!(status_bad_request.is_4xx_client_error());
    assert!(!status_bad_request.is_5xx_server_error());
    assert!(status_bad_request.is_error());

    let status_internal_server_error = HttpStatus::InternalServerError;
    assert!(!status_internal_server_error.is_success());
    assert!(!status_internal_server_error.is_redirection());
    assert!(!status_internal_server_error.is_4xx_client_error());
    assert!(status_internal_server_error.is_5xx_server_error());
    assert!(status_internal_server_error.is_error());
}

#[test]
fn test_http_status_value_of() {
    assert_eq!(HttpStatus::value_of(200), Ok(HttpStatus::Ok));
    assert_eq!(HttpStatus::value_of(400), Ok(HttpStatus::BadRequest));
    assert_eq!(HttpStatus::value_of(404), Ok(HttpStatus::NotFound));
    assert_eq!(
        HttpStatus::value_of(500),
        Ok(HttpStatus::InternalServerError)
    );
    assert_eq!(HttpStatus::value_of(502), Ok(HttpStatus::BadGateway));
    assert_eq!(HttpStatus::value_of(504), Ok(HttpStatus::GatewayTimeout));

    assert_eq!(HttpStatus::resolve(200), Some(HttpStatus::Ok));
    assert_eq!(HttpStatus::resolve(400), Some(HttpStatus::BadRequest));
    assert_eq!(HttpStatus::resolve(404), Some(HttpStatus::NotFound));
    assert_eq!(
        HttpStatus::resolve(500),
        Some(HttpStatus::InternalServerError)
    );
    assert_eq!(HttpStatus::resolve(502), Some(HttpStatus::BadGateway));
    assert_eq!(HttpStatus::resolve(504), Some(HttpStatus::GatewayTimeout));

    assert!(HttpStatus::value_of(600).is_err());
    assert!(HttpStatus::resolve(600).is_none());
}

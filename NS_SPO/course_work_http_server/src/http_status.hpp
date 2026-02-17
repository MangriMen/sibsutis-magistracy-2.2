#pragma once

#include <string>

enum class HttpStatus {
    // 1xx: Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    // 2xx: Success
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ThisIsFine = 218,
    IMUsed = 226,
    // 3xx: Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    ResumeIncomplete = 308,
    // 4xx: Client Error
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
    RequestEntityTooLarge = 413,
    RequestURITooLong = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    PageExpired = 419,
    MethodFailure = 420,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    LoginTimeout = 440,
    ConnectionClosedWithoutResponse = 444,
    RetryWith = 449,
    BlockedByParentalControls = 450,
    UnavailableForLegalReasons = 451,
    RequestHeaderTooLarge = 494,
    SSLCertificateError = 495,
    SSLCertificateRequired = 496,
    HttpRequestSentToHttpsPort = 497,
    InvalidToken = 498,
    ClientClosedRequest = 499,
    // 5xx: Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    BandwidthLimitExceeded = 509,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
    UnknownError = 520,
    WebServerIsDown = 521,
    ConnectionTimedOut = 522,
    OriginIsUnreachable = 523,
    ATimeoutOccurred = 524,
    SSLHandshakeFailed = 525,
    InvalidSSLCertificate = 526,
    RailgunError = 527,
    OriginDNSError = 530,
    NetworkReadTimeout = 598
};

namespace http_status {
inline std::string to_string(HttpStatus status)
{
    switch (status) {
    case HttpStatus::Continue:
        return "Continue";
    case HttpStatus::SwitchingProtocols:
        return "Switching Protocols";
    case HttpStatus::Processing:
        return "Processing";
    case HttpStatus::EarlyHints:
        return "Early Hints";
    case HttpStatus::OK:
        return "OK";
    case HttpStatus::Created:
        return "Created";
    case HttpStatus::Accepted:
        return "Accepted";
    case HttpStatus::NonAuthoritativeInformation:
        return "Non-Authoritative Information";
    case HttpStatus::NoContent:
        return "No Content";
    case HttpStatus::ResetContent:
        return "Reset Content";
    case HttpStatus::PartialContent:
        return "Partial Content";
    case HttpStatus::MultiStatus:
        return "Multi-Status";
    case HttpStatus::AlreadyReported:
        return "Already Reported";
    case HttpStatus::ThisIsFine:
        return "This is fine";
    case HttpStatus::IMUsed:
        return "IM Used";
    case HttpStatus::MultipleChoices:
        return "Multiple Choices";
    case HttpStatus::MovedPermanently:
        return "Moved Permanently";
    case HttpStatus::Found:
        return "Found";
    case HttpStatus::SeeOther:
        return "See Other";
    case HttpStatus::NotModified:
        return "Not Modified";
    case HttpStatus::SwitchProxy:
        return "Switch Proxy";
    case HttpStatus::TemporaryRedirect:
        return "Temporary Redirect";
    case HttpStatus::ResumeIncomplete:
        return "Resume Incomplete";
    case HttpStatus::BadRequest:
        return "Bad Request";
    case HttpStatus::Unauthorized:
        return "Unauthorized";
    case HttpStatus::PaymentRequired:
        return "Payment Required";
    case HttpStatus::Forbidden:
        return "Forbidden";
    case HttpStatus::NotFound:
        return "Not Found";
    case HttpStatus::MethodNotAllowed:
        return "Method Not Allowed";
    case HttpStatus::NotAcceptable:
        return "Not Acceptable";
    case HttpStatus::ProxyAuthenticationRequired:
        return "Proxy Authentication Required";
    case HttpStatus::RequestTimeout:
        return "Request Timeout";
    case HttpStatus::Conflict:
        return "Conflict";
    case HttpStatus::Gone:
        return "Gone";
    case HttpStatus::LengthRequired:
        return "Length Required";
    case HttpStatus::PreconditionFailed:
        return "Precondition Failed";
    case HttpStatus::RequestEntityTooLarge:
        return "Request Entity Too Large";
    case HttpStatus::RequestURITooLong:
        return "Request-URI Too Long";
    case HttpStatus::UnsupportedMediaType:
        return "Unsupported Media Type";
    case HttpStatus::RequestedRangeNotSatisfiable:
        return "Requested Range Not Satisfiable";
    case HttpStatus::ExpectationFailed:
        return "Expectation Failed";
    case HttpStatus::ImATeapot:
        return "I'm a teapot";
    case HttpStatus::PageExpired:
        return "Page Expired";
    case HttpStatus::MethodFailure:
        return "Method Failure";
    case HttpStatus::MisdirectedRequest:
        return "Misdirected Request";
    case HttpStatus::UnprocessableEntity:
        return "Unprocessable Entity";
    case HttpStatus::Locked:
        return "Locked";
    case HttpStatus::FailedDependency:
        return "Failed Dependency";
    case HttpStatus::UpgradeRequired:
        return "Upgrade Required";
    case HttpStatus::PreconditionRequired:
        return "Precondition Required";
    case HttpStatus::TooManyRequests:
        return "Too Many Requests";
    case HttpStatus::RequestHeaderFieldsTooLarge:
        return "Request Header Fields Too Large";
    case HttpStatus::LoginTimeout:
        return "Login Time-out";
    case HttpStatus::ConnectionClosedWithoutResponse:
        return "Connection Closed Without Response";
    case HttpStatus::RetryWith:
        return "Retry With";
    case HttpStatus::BlockedByParentalControls:
        return "Blocked by Windows Parental Controls";
    case HttpStatus::UnavailableForLegalReasons:
        return "Unavailable For Legal Reasons";
    case HttpStatus::RequestHeaderTooLarge:
        return "Request Header Too Large";
    case HttpStatus::SSLCertificateError:
        return "SSL Certificate Error";
    case HttpStatus::SSLCertificateRequired:
        return "SSL Certificate Required";
    case HttpStatus::HttpRequestSentToHttpsPort:
        return "HTTP Request Sent to HTTPS Port";
    case HttpStatus::InvalidToken:
        return "Invalid Token";
    case HttpStatus::ClientClosedRequest:
        return "Client Closed Request";
    case HttpStatus::InternalServerError:
        return "Internal Server Error";
    case HttpStatus::NotImplemented:
        return "Not Implemented";
    case HttpStatus::BadGateway:
        return "Bad Gateway";
    case HttpStatus::ServiceUnavailable:
        return "Service Unavailable";
    case HttpStatus::GatewayTimeout:
        return "Gateway Timeout";
    case HttpStatus::HttpVersionNotSupported:
        return "HTTP Version Not Supported";
    case HttpStatus::VariantAlsoNegotiates:
        return "Variant Also Negotiates";
    case HttpStatus::InsufficientStorage:
        return "Insufficient Storage";
    case HttpStatus::LoopDetected:
        return "Loop Detected";
    case HttpStatus::BandwidthLimitExceeded:
        return "Bandwidth Limit Exceeded";
    case HttpStatus::NotExtended:
        return "Not Extended";
    case HttpStatus::NetworkAuthenticationRequired:
        return "Network Authentication Required";
    case HttpStatus::UnknownError:
        return "Unknown Error";
    case HttpStatus::WebServerIsDown:
        return "Web Server Is Down";
    case HttpStatus::ConnectionTimedOut:
        return "Connection Timed Out";
    case HttpStatus::OriginIsUnreachable:
        return "Origin Is Unreachable";
    case HttpStatus::ATimeoutOccurred:
        return "A Timeout Occurred";
    case HttpStatus::SSLHandshakeFailed:
        return "SSL Handshake Failed";
    case HttpStatus::InvalidSSLCertificate:
        return "Invalid SSL Certificate";
    case HttpStatus::RailgunError:
        return "Railgun Listener to Origin Error";
    case HttpStatus::OriginDNSError:
        return "Origin DNS Error";
    case HttpStatus::NetworkReadTimeout:
        return "Network Read Timeout Error";
    default:
        return "Unknown Status";
    }
}
}
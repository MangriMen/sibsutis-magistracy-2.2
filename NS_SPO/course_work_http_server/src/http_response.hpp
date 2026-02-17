#pragma once

#include "http_status.hpp"
#include <iostream>
#include <map>
#include <sstream>
#include <string>

// Http protocol new line sequence
const std::string NL = "\r\n";

class HttpResponse {
private:
    void build_http_header(std::stringstream& ss) const
    {
        ss << "HTTP/1.1 " << code_ << " " << status_ << NL;
    }

    void build_headers(std::stringstream& ss) const
    {
        for (auto const& [name, value] : headers_) {
            ss << name << ": " << value << NL;
        }
    }

public:
    explicit HttpResponse(HttpStatus status)
        : code_(static_cast<int>(status))
        , status_(http_status::to_string(status))
    {
        add_header("Server", "HttpServer/1.0");
    }

    void add_header(std::string name, std::string value)
    {
        headers_[std::move(name)] = std::move(value);
    }

    void set_body(std::string body, const std::string& content_type = "text/html")
    {
        body_ = std::move(body);
        add_header("Content-Type", content_type);
        add_header("Content-Length", std::to_string(body_.size()));
    }

    std::string to_string() const
    {
        std::stringstream ss;

        build_http_header(ss);
        build_headers(ss);
        ss << NL;

        std::string result = ss.str();

        if (result.find('\0') != std::string::npos) {
            std::cerr << "CRITICAL: Null byte detected in HTTP headers!" << std::endl;
        }

        result.append(body_);
        return result;
    }

private:
    int code_;
    std::string status_;
    std::map<std::string, std::string> headers_;
    std::string body_;
};
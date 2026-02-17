#pragma once

#include "http_response.hpp"
#include "http_status.hpp"
#include "mime_types.hpp"
#include <asio.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <istream>
#include <memory>
#include <string>

class HttpSession : public std::enable_shared_from_this<HttpSession> {
public:
    explicit HttpSession(asio::ip::tcp::socket socket, std::filesystem::path root_path)
        : socket_(std::move(socket))
        , root_path_(std::move(root_path))
    {
    }

    void start()
    {
        handle_request();
    }

private:
    void handle_request()
    {
        auto self = shared_from_this();

        // Read data until the end of HTTP headers (\r\n\r\n)
        asio::async_read_until(socket_, buffer_, "\r\n\r\n",
            [self](std::error_code ec, std::size_t bytes_transferred) {
                if (!ec) {
                    self->process_request(bytes_transferred);
                } else if (ec != asio::error::operation_aborted) {
                    std::cerr << "Read error: " << ec.message() << std::endl;
                }
            });
    }

    std::string read_http_header(std::size_t bytes_transferred)
    {
        std::string data {
            asio::buffers_begin(buffer_.data()),
            asio::buffers_begin(buffer_.data()) + bytes_transferred
        };

        buffer_.consume(bytes_transferred);

        return data;
    }

    void process_request(std::size_t bytes_transferred)
    {

        std::string data = read_http_header(bytes_transferred);

        // Parse header
        std::stringstream ss(data);

        std::string method, path, protocol;

        ss >> method >> path >> protocol;

        if (method == "GET") {
            send_response(path);
        } else {
            send_error(HttpStatus::MethodNotAllowed);
        }
    }

    void send_response(const std::string& http_path)
    {
        auto self = shared_from_this();

        std::filesystem::path abs_root = std::filesystem::absolute(root_path_);

        std::string path = http_path;

        auto query_pos = path.find('?');
        if (query_pos != std::string::npos) {
            path = path.substr(0, query_pos);
        }

        // Replace / to /index.html
        std::string rel_path = path;
        if (!rel_path.empty() && rel_path[0] == '/') {
            rel_path.erase(0, 1);
        }
        if (rel_path.empty()) {
            rel_path = "index.html";
        }

        // Make path absolute
        std::filesystem::path full_path = std::filesystem::weakly_canonical(abs_root / rel_path);

        // Check that path starts from root_path
        auto [root_it, file_it] = std::mismatch(abs_root.begin(), abs_root.end(), full_path.begin());

        if (root_it != abs_root.end() || !std::filesystem::exists(full_path) || std::filesystem::is_directory(full_path)) {
            send_error(HttpStatus::NotFound);
            return;
        }

        std::ifstream file(full_path, std::ios::binary);
        if (!file) {
            send_error(HttpStatus::InternalServerError);
            return;
        }

        // Read file contents
        std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());

        HttpResponse response(HttpStatus::OK);
        response.add_header("Connection", "close");

        std::string ext = full_path.extension().string();
        response.set_body(std::move(content), std::string(mime_types::from_extension(ext)));

        response_data_ = response.to_string();

        asio::async_write(socket_, asio::buffer(response_data_),
            [self](std::error_code ec, std::size_t) {
                if (!ec) {
                    std::error_code ignored_ec;
                    self->socket_.shutdown(asio::ip::tcp::socket::shutdown_both, ignored_ec);
                }
            });
    }

    void send_error(HttpStatus status)
    {
        auto self = shared_from_this();

        HttpResponse response(status);
        response.add_header("Connection", "close");
        response.set_body("<h1>" + std::to_string(static_cast<int>(status)) + " " + http_status::to_string(status) + "</h1>", "text/html");

        response_data_ = response.to_string();

        asio::async_write(socket_, asio::buffer(response_data_), [self](std::error_code ec, std::size_t) {
            if (!ec) {
                std::error_code ignored_ec;
                self->socket_.shutdown(asio::ip::tcp::socket::shutdown_both, ignored_ec);
            }
        });
    }

    asio::ip::tcp::socket socket_;
    asio::streambuf buffer_;
    std::string response_data_;
    std::filesystem::path root_path_;
};

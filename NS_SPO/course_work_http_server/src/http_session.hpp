#pragma once

#include <asio.hpp>
#include <iostream>
#include <memory>
#include <string>

class HttpSession : public std::enable_shared_from_this<HttpSession> {
public:
    explicit HttpSession(asio::ip::tcp::socket socket)
        : socket_(std::move(socket))
    {
    }

    void start()
    {
        read_request();
    }

private:
    void read_request()
    {
        auto self = shared_from_this();
        // Read data until the end of HTTP headers (\r\n\r\n)
        asio::async_read_until(socket_, buffer_, "\r\n\r\n",
            [self](std::error_code ec, std::size_t bytes_transferred) {
                if (!ec) {
                    self->process_request();
                }
            });
    }

    void process_request()
    {
        // Convert buffer to string for manual parsing
        std::string data {
            asio::buffers_begin(buffer_.data()),
            asio::buffers_begin(buffer_.data()) + buffer_.size()
        };

        std::cout << "Received request:\n"
                  << data << std::endl;

        std::string method, path;
        size_t method_end = data.find(' ');
        if (method_end != std::string::npos) {
            method = data.substr(0, method_end);
            size_t path_end = data.find(' ', method_end + 1);
            if (path_end != std::string::npos) {
                path = data.substr(method_end + 1, path_end - method_end - 1);
            }
        }

        if (method == "GET") {
            send_response(path);
        } else {
            send_error(405, "Method Not Allowed");
        }
    }

    void send_response(const std::string& path)
    {
        auto self = shared_from_this();
        // Simple router logic
        std::string content = "<html><body><h1>Path: " + path + "</h1></body></html>";

        std::string response = "HTTP/1.1 200 OK\r\n"
                               "Content-Type: text/html\r\n"
                               "Content-Length: "
            + std::to_string(content.size()) + "\r\n"
                                               "Connection: close\r\n"
                                               "\r\n"
            + content;

        asio::async_write(socket_, asio::buffer(response),
            [self](std::error_code ec, std::size_t) {
                if (!ec) {
                    // Shutdown the socket after sending
                    self->socket_.shutdown(asio::ip::tcp::socket::shutdown_both);
                }
            });
    }

    void send_error(int code, std::string msg)
    {
        // Logic for 404, 405 etc.
    }

    asio::ip::tcp::socket socket_;
    asio::streambuf buffer_;
};
#pragma once

#include "http_session.hpp"
#include <asio.hpp>
#include <filesystem>

class HttpServer {
public:
    HttpServer(asio::io_context& io_context, short port, std::filesystem::path root_path)
        : acceptor_(io_context, asio::ip::tcp::endpoint(asio::ip::tcp::v6(), port))
        , root_path_(std::move(root_path))
    {
        print_hello_message();
        start_accept();
    }

private:
    void print_hello_message()
    {
        std::cout << "Server listening on port: " << acceptor_.local_endpoint().port() << std::endl;
    }

    void start_accept()
    {
        acceptor_.async_accept(
            [this](std::error_code ec, asio::ip::tcp::socket socket) {
                if (!ec) {
                    std::make_shared<HttpSession>(std::move(socket), root_path_)->start();
                }
                start_accept();
            });
    }

    asio::ip::tcp::acceptor acceptor_;
    std::filesystem::path root_path_;
};
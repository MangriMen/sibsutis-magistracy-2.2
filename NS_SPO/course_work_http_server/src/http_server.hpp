#pragma once

#include "http_session.hpp"
#include <asio.hpp>

class HttpServer {
public:
    HttpServer(asio::io_context& io_context, short port)
        : acceptor_(io_context, asio::ip::tcp::endpoint(asio::ip::tcp::v6(), port))
    {
        start_accept();
    }

private:
    void start_accept()
    {

        acceptor_.async_accept(
            [this](std::error_code ec, asio::ip::tcp::socket socket) {
                if (!ec) {
                    std::make_shared<HttpSession>(std::move(socket))->start();
                }
                start_accept();
            });
    }

    asio::ip::tcp::acceptor acceptor_;
};
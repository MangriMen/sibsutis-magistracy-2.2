#include "http_server.hpp"
#include <iostream>

int main()
{
    try {
        asio::io_context ioc;

        HttpServer(ioc, 8080);

        ioc.run();
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }

    return 0;
}
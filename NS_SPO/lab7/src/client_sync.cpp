#include <boost/asio.hpp>
#include <chrono>
#include <iostream>
#include <thread>

using namespace boost::asio;
using ip::tcp;

int main(int argc, char* argv[])
{
    if (argc != 3) {
        std::cerr << "Usage: client <IP> <PORT>" << std::endl;
        return 1;
    }

    try {
        io_context io_ctx;
        tcp::socket socket(io_ctx);

        tcp::resolver resolver(io_ctx);
        auto endpoints = resolver.resolve(argv[1], argv[2]);

        connect(socket, endpoints);
        std::cout << "Connected to " << argv[1] << ":" << argv[2] << std::endl;

        int i;
        std::cout << "Enter a number (1-10): ";
        std::cin >> i;
        if (i < 1 || i > 10)
            i = 1;

        while (true) {
            write(socket, buffer(&i, sizeof(int)));

            int result = 0;
            read(socket, buffer(&result, sizeof(int)));

            std::cout << "Sent: " << i << " | Received square: " << result << std::endl;

            std::this_thread::sleep_for(std::chrono::seconds(i));
        }

    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}
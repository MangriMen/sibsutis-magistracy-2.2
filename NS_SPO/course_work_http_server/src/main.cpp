#include "http_server.hpp"
#include <iostream>

int main(int argc, char* argv[])
{
    try {
        if (argc < 3) {
            std::cerr << "Usage: " << argv[0] << " " << "<port> <root_path>\n";
            return 1;
        }

        unsigned short port = static_cast<unsigned short>(std::atoi(argv[1]));
        std::filesystem::path root = std::filesystem::absolute(argv[2]);

        if (!std::filesystem::exists(root) || !std::filesystem::is_directory(root)) {
            std::cerr << "Error: Root path does not exist or is not a directory: " << root << "\n";
            return 1;
        }

        asio::io_context ioc;

        HttpServer server(ioc, port, root);

        ioc.run();
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }

    return 0;
}
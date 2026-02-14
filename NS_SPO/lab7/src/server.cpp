#include <boost/asio.hpp>
#include <chrono>
#include <iostream>
#include <memory>

using namespace boost::asio;
using ip::tcp;

class Session : public std::enable_shared_from_this<Session> {
public:
    explicit Session(tcp::socket socket)
        : socket_(std::move(socket))
    {
    }

    void start()
    {
        on_client_connected();
        do_read();
    }

    ~Session()
    {
        log_client_disconnected();
    }

private:
    void do_read()
    {
        auto self(shared_from_this());
        socket_.async_read_some(buffer(&input_number_, sizeof(int)),
            [this, self](boost::system::error_code ec, std::size_t) {
                if (!ec) {
                    int result = get_client_answer(input_number_);
                    log_client_answer(result);
                    do_write(result);
                }
            });
    }

    void do_write(int result)
    {
        auto self(shared_from_this());
        async_write(socket_, buffer(&result, sizeof(result)),
            [this, self](boost::system::error_code ec, std::size_t) {
                if (!ec) {
                    do_read(); // Continue listening
                }
            });
    }

    int get_client_answer(int input)
    {
        return input * input;
    }

    void log_client_answer(int result)
    {
        std::cout << client_address_ << ":" << client_port_ << " " << result << " -> " << result << std::endl;
    }

    void on_client_connected()
    {
        try {
            client_address_ = socket_.remote_endpoint().address().to_string();
            client_port_ = socket_.remote_endpoint().port();
        } catch (boost::system::system_error& e) {
            std::cerr << "Exception: " << e.what() << std::endl;
        }

        log_client_connected();
    }

    void log_client_connected()
    {
        std::cout << "[+]: " << client_address_ << ":" << client_port_ << std::endl;
    }

    void log_client_disconnected()
    {
        std::cout << "[-]: " << client_address_ << ":" << client_port_ << std::endl;
    }

    tcp::socket socket_;
    int input_number_ = 0;

    std::string client_address_;
    unsigned short client_port_ = 0;
};

class Server {
public:
    Server(io_context& io_ctx, short port)
        : acceptor_(io_ctx, tcp::endpoint(tcp::v6(), port))
    {

        print_hello_msg();
        do_accept();
    }

private:
    void do_accept()
    {
        acceptor_.async_accept(
            [this](boost::system::error_code ec, tcp::socket socket) {
                if (!ec) {
                    std::make_shared<Session>(std::move(socket))->start();
                }
                do_accept();
            });
    }

    void print_hello_msg()
    {
        std::cout << "Server listening on port: " << acceptor_.local_endpoint().port() << std::endl;
    }

    tcp::acceptor acceptor_;
};

int main()
{
    try {
        io_context io_ctx;

        Server server(io_ctx, 0);

        io_ctx.run();
    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}
#include <boost/asio.hpp>
#include <iostream>
#include <memory>

using namespace boost::asio;
using ip::tcp;

class AsyncClient : public std::enable_shared_from_this<AsyncClient> {
public:
    AsyncClient(io_context& io_ctx, const std::string& host, const std::string& port, int number)
        : resolver_(io_ctx)
        , socket_(io_ctx)
        , timer_(io_ctx)
        , i_(number)
    {
        endpoints_ = resolver_.resolve(host, port);
    }

    void start()
    {
        do_connect();
    }

private:
    void do_connect()
    {
        auto self(shared_from_this());
        async_connect(socket_, endpoints_,
            [this, self](boost::system::error_code ec, tcp::endpoint) {
                if (!ec) {
                    std::cout << "Connected! Starting loop..." << std::endl;
                    do_write();
                }
            });
    }

    void do_write()
    {
        auto self(shared_from_this());
        async_write(socket_, buffer(&i_, sizeof(int)),
            [this, self](boost::system::error_code ec, std::size_t) {
                if (!ec)
                    do_read();
            });
    }

    void do_read()
    {
        auto self(shared_from_this());
        async_read(socket_, buffer(&result_, sizeof(int)),
            [this, self](boost::system::error_code ec, std::size_t) {
                if (!ec) {
                    std::cout << "Received square: " << result_ << std::endl;
                    waitForNextTick();
                }
            });
    }

    void waitForNextTick()
    {
        auto self(shared_from_this());
        timer_.expires_after(std::chrono::seconds(i_));
        timer_.async_wait([this, self](boost::system::error_code ec) {
            if (!ec)
                do_write();
        });
    }

    tcp::resolver resolver_;
    tcp::socket socket_;
    steady_timer timer_;
    tcp::resolver::results_type endpoints_;
    int i_;
    int result_ = 0;
};

int asc_number()
{
    int number;

    std::cout << "Enter number (1-10): ";
    std::cin >> number;

    return number;
}

int main(int argc, char* argv[])
{
    if (argc != 3)
        return 1;

    try {
        io_context io_ctx;

        int number = asc_number();

        std::make_shared<AsyncClient>(io_ctx, argv[1], argv[2], number)->start();

        io_ctx.run();

    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}
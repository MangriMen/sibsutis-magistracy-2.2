#include <iostream>
#include <string>

// Returns pair {start, end}. If end == 0, read to end.
std::pair<long long, long long> parse_range(const std::string& range_header, long long file_size)
{
    try {
        size_t eq_pos = range_header.find('=');
        size_t dash_pos = range_header.find('-');
        if (eq_pos == std::string::npos || dash_pos == std::string::npos)
            return { 0, -1 };

        std::string s_start = range_header.substr(eq_pos + 1, dash_pos - eq_pos - 1);
        std::string s_end = range_header.substr(dash_pos + 1);

        long long start = s_start.empty() ? 0 : std::stoll(s_start);
        long long end = s_end.empty() ? file_size - 1 : std::stoll(s_end);

        // Защита от выхода за границы
        if (start >= file_size)
            start = file_size - 1;
        if (end >= file_size)
            end = file_size - 1;
        if (start < 0)
            start = 0;

        return { start, end };
    } catch (...) {
        return { 0, -1 }; // В случае любой ошибки парсинга отдаем весь файл
    }
}
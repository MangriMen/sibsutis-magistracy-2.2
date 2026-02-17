namespace mime_types {
inline std::string from_extension(const std::string& ext)
{
    static const std::map<std::string, std::string> types = {
        { ".html", "text/html" },
        { ".css", "text/css" },
        { ".js", "application/javascript" },
        { ".png", "image/png" },
        { ".jpg", "image/jpeg" }
    };

    auto it = types.find(ext);
    if (it != types.end()) {
        return it->second;
    }
    return "application/octet-stream";
}
}
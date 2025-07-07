// Файл, который лежит в src/bin/*.rs образует crate-исполняемый файл (main)
// Из main можно пользоваться только тем, что выставлено наружу (pub) из библиотечного crate и корректно объявлено в lib.rs
use backend::lib::parse_funcs::parse_request;
fn main()
{
    let raw_requests = vec![
    "GET /api/status HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
\r\n"
        .to_string(),
    "GET / HTTP/1.1\r\n\
Host: localhost\r\n\
\r\n"
        .to_string(),
    "POST /api/users HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
Content-Length: 27\r\n\
\r\n\
{\"name\":\"alice\",\"age\":30}"
        .to_string(),
    "PUT /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
Authorization: Bearer TOKEN123\r\n\
Content-Type: application/json\r\n\
Accept: */*\r\n\
\r\n\
{\"price\":19.99,\"stock\":100}"
        .to_string(),
    "DELETE /api/items/42 HTTP/1.1\r\n\
Host: api.example.com\r\n\
X-Debug-Mode: true\r\n\\
r\n"
        .to_string(),
    "GET /search?q=rust+lang&sort=desc HTTP/2.0\r\n\
Host: www.example.com\r\n\
User-Agent: MyClient/1.0\r\n\
Accept: text/html,application/xhtml+xml\r\n\
Cookie: session=abcd1234; theme=dark\r\n\
\r\n"
        .to_string(),

    "POST /login HTTP/1.1\r\n\
Host: auth.example.com\r\n\
Content-Type: application/x-www-form-urlencoded\r\n\
Content-Length: 29\r\n\
Cookie: mobile=true\r\n\
\r\n\
username=foo&password=bar"
        .to_string(),
    ];

    // 1 - нет
    // 2 - нет
    // 3 - да (Json)
    // 4 - да (Json)
    // 5 - нет
    // 6 - нет
    // 7 - да (Plain)

    for raw in &raw_requests
    {
        match parse_request(raw.clone())
        {
            Ok(req)  => println!("OK: {:?}", req),
            Err(err) => eprintln!("Parse error: {:?}", err),
        }
    }
    
}
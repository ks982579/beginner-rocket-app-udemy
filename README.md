# Beginner APIs in Rust

This is just following a Udemy course to practice building web applications in Rust. We will use Rocket for now as it should have a good DX. 

How to include dependencies? I like to use `cargo add` as such

```bash
cargo add rocket --features=json
cargo add serde_json
```

HTTP requests should be as follows:
+ GET
+ POST
+ PUT
+ DELETE

In the POST request, you notice we also inclued `format = "json"`.
This is to tell rust to include that format in the HTTP header. 

Routing is also nice in that, we pass in functions.

```bash
curl 127.0.0.1:8000/rustaceans -X POST -H 'Content-type: application/json'
curl 127.0.0.1:8000/rustaceans/42 -X DELETE -I
```

Seems like the workflow is create a function endpoint with proper macros.
Then, you _register_ it in the main function.
For the actual routes, we _mount_ them using the `routes!` macro.
We then register our errors using the `catchers!` macro.

HTTP Standard of **Basic Access Authentication** (wiki). 
In the HTTP header, you pass in something like `Authorization: Basic <credentials>`.

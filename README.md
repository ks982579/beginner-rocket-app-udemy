# Beginner APIs in Rust

[Moergo Glove](https://www.moergo.com/)

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
There are other types of authorization like "bearer", but we do not look at it ATM.
Wiki with the example, you use a colon to separate the username and password an then 
base64 encode it. 
Base64 can be decoded.
The encoding isn't for security but for transporting over the internet. 
You want HTTPS enabled, and you don't want your basic credentials logged anywhere people can find it. 

In Rust, the `?` tells Rust that if the expression returns a `None` value, stop and return `None` there.
Else, if it returns a `Some` value, continue through the function. 

Also, must deviate slightly from the Udemy course here because for `base64::decode` I am getting
a deprecated warning and instructed to use `Engine::decode`. 
Reading through the [base64 docs](https://docs.rs/base64/latest/base64/), you can import the
prelude and then use `BASE64_STANDARD.decode()?`.
Else, you have to instantiate your own engine with configuration.

What is Base64? Check out [base64encoder.io](https://www.base64encoder.io/learn/). 
It's purpose is to encode binary or non-ASCII text data to printable ASCII format so
it can be safely transmitted over _any_ communication channel. 

For our application, the header would be "Authorization: Basic Base64Encoding=="
Where that encoding is "username:password" separated by a colon like so.

What about Request Guards? Sounds like Middleware.
There are apparently 2 versions of [rocket outcome](https://docs.rs.rocket/latest/rocket/outcome/)
One being `rocket:outcome::Outcome::{Success, Error, Forward}` which is the OG.
The second comes from `rocket::request::Outcome` which is _type alias_ for the 
actual Outcome, and only requires Success and Forward. 

I'm not 100% sure, but it seems like when you implement the `FromRequest` trait,
then the struct becomes a trait guard. 
Maybe in the macros, they try to pull the value from the HTTP request, 
hense the name...
Very interesting implementation. 

Yes, so we add the `BasicAuth` struct as a parameter and it
somehow automagically runst the `from_request` method that
returns Unauthorized if it is infact, unauthorized. 

## Adding Database Support

We will use Diesel, which requires the following:

```bash
sudo apt-get install libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite

diesel setup --database-url ./database.sqlite
diesel migration generate create_rustaceans
```

The former gets SQLite ready for our system, and the latter installs CLI tools only for SQLite.
Check the Diesel website, but I think it comes with PostGreSQL and MySQL support.

Then, we use the CLI to setup a database, which creates it and a `diesel.toml` file.
We also create migrations for it.
This creates `down.sql` and `up.sql` files. 
I think it slightly defeats the purpose, but you put SQL into those files for when the databse is run.
Like, `up.sql` is for spinning up the database...

Once you have those set up, you can check migrations...

```bash
diesel migration list --database-url=database.sqlite
diesel migration run --database-url=./database.sqlite/
```

it is also good to run the `migration revert` command to ensure you can roll back.
You will see a `schema.rs` file now in `./src`. 
There's a macro that converts things to structs! It's cool.

You will need to bring in diesel at this point. The `r2d2` feature is meant to be more
efficient at keeping connections open instead of creating a new connection for every query.
Rocket also needs its own dependency.


https://rocket.rs/v0.4/guide/requests/ - request queries

let callback = |req: &Request, mut response: Response| {
                    println!("Received a new ws handshake");
                    println!("The request's path is: {}", req.uri().query().unwrap());
                    println!("The request's headers are:");
                    for (ref header, _value) in req.headers() {
                        println!("* {}", header);
                    }
    
                    // Let's add an additional header to our response to the client.
                    let headers = response.headers_mut();
                    headers.append("MyCustomHeader", ":)".parse().unwrap());
                    headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());
    
                    Ok(response)
                };

let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();



















# Svelte-on-Rust

Starter template for [Svelte](https://svelte.dev) frontend apps with Rust [Rocket](https://rocket.rs) backend server. 



## Requirements

NodeJs - [Install](https://nodejs.org/en/download/)

Rust  - [Install](https://www.rust-lang.org/tools/install) 

Rust Nightly for the project folder


## Get started
Create a new project based on this template using [degit](https://github.com/Rich-Harris/degit) and 
install the dependencies...

```bash
npx degit sachinbhutani/svelte-on-rust svelte-rocket
cd svelte-rocket
rustup override set nightly
npm install
```


...then start Rocket server and [Rollup](https://rollupjs.org) in two different terminals 

Terminal 1: (To run the rust server)
```bash
cargo run  
```
Terminal 2: (To build and hot reload svelte components)
```bash
npm run dev  
```

Navigate to [localhost:8000](http://localhost:8000). You should see your app running. 
All svelte component live in `client` directory. Save any changes live-reloading.
All Rocket code lives in `src` directory. To rebuild Rust code use cargo run after saving your changes. 
All static files are served from `public` direcotry. Including the JS code compiled by Svelte Compiler.


## Building and running in production mode

To create an optimised version of the app:

```bash
npm run build
cargo build
```

## Built With
[Rocket](https://rocket.rs/) 

[Svelte](https://svelte.dev/)

[YRV](https://github.com/pateketrueke/yrv) 

[Bulma](https://bulma.io)

## Change Log

v0.1.4: update packages  because `cargo run` failed 

v0.1.3: Added authentication example with private cookies

v0.1.2: Added Bulma CSS styling
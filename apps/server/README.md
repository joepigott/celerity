# `celerityd`

This is the reference implementation for the 
[`celerity`](https://github.com/joepigott/celerity) server. The library 
implements most of the server functionality, and essentially only leaves the 
runtime and threading up to the server implementation. This reference uses 
`tokio`.

## Usage

`celerityd` expects a configuration file at `/etc/celerityd/config.toml`. This 
must include a `[server]` section with the address and port to listen on, as 
well as a `[scheduler]` section with the storage path, write timeout in 
minutes, and scheduler timeout in milliseconds. An example configuration could 
be:
```toml
# /etc/celerityd/config.toml

[server]
address = "127.0.0.1:12345"

[scheduler]
data_path = "/var/lib/celerityd/tasks.json"
write_timeout = 10
scheduler_timeout = 1000
```

## Serving and Encryption

If serving over the internet, it is **strongly** recommended to encrypt the
communication between the client and server. The cleanest way to do this is
through a reverse proxy such as nginx. As a minimal working example, your
nginx configuration file could be
```
server {
    listen 80;
    server_name domain.name;

    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl;
    server_name domain.name;

    ssl_certificate /path/to/fullchain.pem;
    ssl_certificate_key /path/to/privkey.pem;

    location / {
        # set headers, body size, etc.
        proxy_pass http://localhost:port; # same port as celerityd configuration
    }
}
```
Nginx will enforce https, while `celerityd` serves locally over http. This is 
clean, easy, and by far the recommended way to encrypt the communication.

### `https` feature

If you are unable to use a reverse proxy, `celerityd` does natively support https
through the `https` crate feature. You must build the crate using
`--features=https` and obtain a certificate and its private key, then define 
their paths in the configuration file's `server` block. For example,
```toml
# /etc/celerityd/config.toml

[server]
address = "127.0.0.1:12345"
cert_path = "/path/to/cert.pem"
key_path = "/path/to/privkey.pem"

[scheduler]
data_path = "/var/lib/celerityd/tasks.json"
write_timeout = 10
scheduler_timeout = 1000
```
Again, I *strongly* recommend using a reverse proxy over the native https 
support. This should only be used as a last resort, as it's much less flexible.

#### Folder for local TLS certificate and key (both files must have the .pem extension)
##### _(just put them in this folder)_

#### How to run dev build with the TLS
```
CERT="webstack/tls/cert.pem" \
KEY="webstack/tls/key.pem" \
cargo leptos serve
```
# Enigma


## Todo / Could do
- [ ] All of it
- [ ] Backend
- [ ] frontend



- Quick backend call sample:

  - /hello:

```bash
curl http://localhost:9000/hello/potato
Hello, potato!
```

  - scrumble > Will be the endpoint to decrypt/encrypt

```bash
curl --location 'localhost:9000/scrumble' \
--header 'Content-Type: application/json' \
--data '{"rotor": 1, "text_in": "wewe", "text_out": "wqrfwef"}'
```

  - Test CORS:

```bash
curl -v --request OPTIONS 'http://127.0.0.1:8000' -H 'Origin: http://aetes.greece.local' -H 'Access-Control-Request-Method: GET'
```

<details>

<summary>Test CORS with response</summary>

```bash
curl -v --request OPTIONS 'http://127.0.0.1:9000' -H 'Origin: http://localhost/scrumble' -H 'Access-Control-Request-Method: PUT'

*   Trying 127.0.0.1:9000...
* Connected to 127.0.0.1 (127.0.0.1) port 9000 (#0)
> OPTIONS / HTTP/1.1
> Host: 127.0.0.1:9000
> User-Agent: curl/7.76.1
> Accept: */*
> Origin: http://localhost/scrumble
> Access-Control-Request-Method: PUT
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< access-control-allow-headers: allow_origin, referer, allow_any_origin, control-request-headers, content-type, access-control-allow-origin
< access-control-allow-methods: POST, PATCH, OPTIONS, HEAD, PUT, GET, DELETE
< access-control-max-age: 300
< access-control-allow-origin: http://localhost/scrumble
< content-length: 0
< date: Tue, 25 Nov 2025 15:12:29 GMT
<
* Connection #0 to host 127.0.0.1 left intact

```
</details>


> [!NOTE]
> All done mostly to learn & play with Rust... (⌒‿⌒)/
> Haven't played with web framework yew in a while, need to refresh my memory





## Acknowledgments & Reference

[Yew](https://yew.rs/docs/tutorial)

[Yew examples](https://github.com/yewstack/yew/tree/master/examples)

[FrancescoXX](https://github.com/FrancescoXX/rust-fullstack-app/blob/main/frontend/src/main.rs)

[Html interaction web_sys](https://docs.rs/web-sys/latest/web_sys/)

* Enigma:

[Online tools](https://cryptii.com/pipes/enigma-machine)

https://stackoverflow.com/questions/76567463/rust-yew-rs-dropdown-component
[![License: WTFPL]

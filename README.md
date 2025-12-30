# Enigma


## Todo / Could do
- [ ] All of it
- [ ] Backend
- [ ] frontend



## RunIt

### Backend
```bash
make dev
```
### Frontend
```bash
trunk serve
```

## Tests and other stuff:
- Quick backend call sample:

```bash
curl http://localhost:9000/hello/potato
Hello, potato!
```

  - scrumble > Will be the endpoint to decrypt/encrypt

```bash

curl --location 'localhost:9000/scrumble' --header 'Content-Type: application/json' --data '{"cryptic":"","plain":"dsada","reflector":{"definition":"AY BR CU DH EQ FS GL IP JX KN MO TZ VW","id":0,"name":"Reflector B"},"rotor":[0,0,0]}'

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


## Acknowledgments & Reference

[Yew](https://yew.rs/docs/tutorial)
[Yew examples](https://github.com/yewstack/yew/tree/master/examples)
[FrancescoXX](https://github.com/FrancescoXX/rust-fullstack-app/blob/main/frontend/src/main.rs)
[Html interaction web_sys](https://docs.rs/web-sys/latest/web_sys/)

## Enigma:
### Testing:

[Cryptii](https://cryptii.com/pipes/enigma-machine) >> lmhrp kh
[CacheSleuth](https://www.cachesleuth.com/enigma.html) >> LMHRPKH
[Berling Physik](https://people.physik.hu-berlin.de/~palloks/js/enigma/enigma-m4_v16_en.html) >> LMHR PKH
[Cryptool](https://www.cryptool.org/en/cto/enigma/) >> LMHRPKH

https://github.com/cryptii/cryptii/blob/main/src/Encoder/Enigma.js



curl --location 'localhost:9000/scrumble' --header 'Content-Type: application/json' --data '{"cryptic":"nrupbob","plain":"welcomedsaljdlasknfsdkfjnsakjeflekaw;nf;wklenfw;lknfsl;fknsdfs","reflector":{"definition":"YRUHQSLDPXNGOKMIEBFZCWVJAT","id":4,"model":"M3","name":"Reflector B"},"rotor":[{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"},{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"},{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"}]}'


 curl --location 'localhost:9000/scrumble' --header 'Content-Type: application/json' --data '{"cryptic":"nrupbob","plain":"welcome","reflector":{"definition":"YRUHQSLDPXNGOKMIEBFZCWVJAT","id":4,"model":"M3","name":"Reflector B"},"rotor":[{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"},{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"},{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I"}]}'

RFL  W.4   W.3   W.2   W.1   ETW   PLG
┌- x <-- i <-- q <-- r <-- w <-- w <-- w (IN)
└> r --> p --> h --> l --> b --> b --> b (OUT)

# Enigma I — Table-Style Trace
Configuration (fixed)
Left rotor: III — BDFHJLCPRTXVZNYEIWGAKMUSQO
Middle rotor: II — AJDKSIRUXBLHWTMCQGZNPYFVOE
Right rotor: I — EKMFLGDQVZNTOWYHXUSPAIBRCJ
Reflector: B — YRUHQSLDPXNGOKMIEBFZCWVJAT
Ring settings: A A A
Plugboard: none

## Rotor positions
| Rotor       | Before key | After stepping |
| ----------- | ---------- | -------------- |
| Left (III)  | A          | A              |
| Middle (II) | A          | A              |
| Right (I)   | A          | **B**          |

Letter mappings (A=0 … Z=25)
| Stage       | Letter | Index |
| ----------- | ------ | ----- |
| Key pressed | W      | 22    |

## Forward path (right → left)
🔹 Right Rotor — I (pos B, offset +1)
| Operation         | Value      |
| ----------------- | ---------- |
| Input index       | 22         |
| + position offset | 23 (X)     |
| Wiring X → R      | 17         |
| − offset          | **16 (Q)** |

🔹 Middle Rotor — II (pos A, offset 0)
| Operation    | Value      |
| ------------ | ---------- |
| Input        | Q (16)     |
| Wiring Q → Q | **16 (Q)** |

🔹 Left Rotor — III (pos A, offset 0)
| Operation    | Value     |
| ------------ | --------- |
| Input        | Q (16)    |
| Wiring Q → I | **8 (I)** |

Reflector
| Input | Output     |
| ----- | ---------- |
| I (8) | **P (15)** |

## Reverse path (left → right)
🔹 Left Rotor — III (reverse wiring)
| Operation     | Value     |
| ------------- | --------- |
| Input         | P (15)    |
| Reverse P → H | **7 (H)** |

Input = W ➞[Rotor 1]➞ Q ➞[Rotor 2]➞ Q ➞[Rotor 3]➞ I ➞[Reflector]➞
P ➞[Rotor 3]➞ H ➞[Rotor 2]➞ L ➞[Rotor 1]➞ B = Output


🔹 Middle Rotor — II (reverse wiring)
| Operation     | Value      |
| ------------- | ---------- |
| Input         | H (7)      |
| Reverse H → L | **11 (L)** |

🔹 Right Rotor — I (reverse, pos B)
| Operation     | Value     |
| ------------- | --------- |
| Input         | L (11)    |
| + offset      | 12 (M)    |
| Reverse M → C | 2         |
| − offset      | **1 (B)** |

EKMFLGDQVZNTOWYHXUSPAIBRCJ
ABCDEFGHIJKLMNOPQRSTUVWXYZ


Final output
| Result    | Letter |
| --------- | ------ |
| Encrypted | **B**  |

out = ( wiring[ (in + P) mod 26 ] − P ) mod 26

out = ( inverse_wiring[ (in + P) mod 26 ] − P ) mod 26
WELCOME 
BFNLZKH
### Documentations:

[Rotors Details](https://en.wikipedia.org/wiki/Enigma_rotor_details)


[![License: WTFPL](https://upload.wikimedia.org/wikipedia/commons/f/fa/WTFPL_badge.png)](/LICENSE.txt)

# Enigma

Just a simulator build as a pet project to work with Yew & Rust.

Note: I have little interest in css, so the presentation of the frontend is not fancy, not at all.


## Potential new pet project...
 Why crypto breaks” micro-lab

 Build minimal demos of famous failures:

 Caesar with frequency attack

 Vigenère with Kasiski

 Enigma with known plaintext

 Each demo ≤ 100 lines.

 Great closure to the Enigma arc.


## Todo / Could do
- [X] Issues when my 3rd rotor ticks - identified, seems related to the double stepping effect...
- [X] Add Plugboard
- [X] Rotors start position, matches one simulator not sure about the others
- [X] Remove special characters
- [X] Allow decryption
- [X] Add the full definition of the settings in the logs
- [X] Turn message to Vector


## Issue detected:


<details>

  <summary>## Issue 001: Double Stepping ✔ </summary>

```
with, rotor I, II, III Enigma I
reflector B
Plain text:
DONOTGOGENTLEINTOTHATGOODNIGHTOLDAGESHOULDBURNANDRAVEATCLOSEOFDAYRAGERAGEAGAINSTTHEDYINGOFTHELIGHT

CORRECT >
OLLZLCBDGWWQSOACEKZQAZUDZQSTYLXUNWHLRXMAQGJCSESXJIYNAPMBOZYLUKRSEAUZHFWXCDNNCMPEPUKAFXZJJMVTUCXARE
OLLZLCBDGWWQSOACEKZQAZUDZQSTYLXUNWHLRXMAQGJCSESXJIYNAPMBOZYLUKRSEAUZHFWXCDNNCMPEPUKAFXZJJMVTUCZMYU
< NOT OK
```

Results:

```
Others:
94 Turned wheels from PDA to QDAInput = L ➞[Rotor 1]➞ U ➞[Rotor 2]➞ S ➞[Rotor 3]➞ G ➞[Reflector]➞ L ➞[Rotor 3]➞ F ➞[Rotor 2]➞ C ➞[Rotor 1]➞ C = Output
95 Turned wheels from QDA to REAInput = I ➞[Rotor 1]➞ S ➞[Rotor 2]➞ B ➞[Rotor 3]➞ D ➞[Reflector]➞ H ➞[Rotor 3]➞ D ➞[Rotor 2]➞ H ➞[Rotor 1]➞ X = Output
96 Turned wheels from REA to SFBInput = G ➞[Rotor 1]➞ K ➞[Rotor 2]➞ X ➞[Rotor 3]➞ P ➞[Reflector]➞ I ➞[Rotor 3]➞ D ➞[Rotor 2]➞ A ➞[Rotor 1]➞ A = Output
97 Turned wheels from SFB to TFBInput = H ➞[Rotor 1]➞ L ➞[Rotor 2]➞ L ➞[Rotor 3]➞ Y ➞[Reflector]➞ A ➞[Rotor 3]➞ Z ➞[Rotor 2]➞ U ➞[Rotor 1]➞ R = Output
98 Turned wheels from TFB to UFBInput = T ➞[Rotor 1]➞ C ➞[Rotor 2]➞ P ➞[Rotor 3]➞ H ➞[Reflector]➞ D ➞[Rotor 3]➞ O ➞[Rotor 2]➞ I ➞[Rotor 1]➞ E = Output

Me:
92 PDA [E] ↣ I Enigma I [A] ↣ II Enigma I [H] ↣ III Enigma I [P] ⟲ Reflector B [P] ↢ III Enigma I [Q] ↢ II Enigma I [K] ↢ I Enigma I ↢ U
93 QDA [L] ↣ I Enigma I [U] ↣ II Enigma I [S] ↣ III Enigma I [G] ⟲ Reflector B [G] ↢ III Enigma I [F] ↢ II Enigma I [C] ↢ I Enigma I ↢ C
94 REB [I] ↣ I Enigma I [S] ↣ II Enigma I [B] ↣ III Enigma I [E] ⟲ Reflector B [E] ↢ III Enigma I [H] ↢ II Enigma I [G] ↢ I Enigma I ↢ Z
95 SEB [G] ↣ I Enigma I [K] ↣ II Enigma I [I] ↣ III Enigma I [S] ⟲ Reflector B [S] ↢ III Enigma I [R] ↢ II Enigma I [T] ↢ I Enigma I ↢ M
96 TEB [H] ↣ I Enigma I [L] ↣ II Enigma I [Y] ↣ III Enigma I [N] ⟲ Reflector B [N] ↢ III Enigma I [E] ↢ II Enigma I [B] ↢ I Enigma I ↢ Y
97 UEB [T] ↣ I Enigma I [C] ↣ II Enigma I [N] ↣ III Enigma I [X] ⟲ Reflector B [X] ↢ III Enigma I [T] ↢ II Enigma I [E] ↢ I Enigma I ↢ U

```
Issue is related to the double stepping, need to review this.

</details>

<details>

<summary>## Issue 002: Non A-B char in set / foreign language ✔ </summary>

```bash  
thread 'tokio-runtime-worker' (205303) panicked at backend/src/punch.rs:74:36:
assertion failed: self.is_char_boundary(n)
```

</details>


## Run the project

[First, get started with Rust](https://rust-lang.org/learn/get-started/)


### Backend

Listens on port 9000.

```bash
cd ~/backend
make dev
```
### Frontend

Listens on port 8000.

```bash
cd ~/frontend
trunk serve
```

## Tests and other stuff:
- Quick backend call sample:


```bash

curl --location 'localhost:9000/scrumble' \
--header 'Content-Type: application/json' \
--data '{"rotor":[{"id":0,"name":"I","definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","model":"Enigma I","notch":["Q"]},{"id":1,"name":"II","definition":"AJDKSIRUXBLHWTMCQGZNPYFVOE","model":"Enigma I","notch":["E"]},{"id":2,"name":"III","definition":"BDFHJLCPRTXVZNYEIWGAKMUSQO","model":"Enigma I","notch":["V"]}],"plain":"hello","cryptic":"","reflector":{"id":4,"name":"Reflector B","definition":"YRUHQSLDPXNGOKMIEBFZCWVJAT","model":"M3"},"start_position":["A","A","A"],"plugboard":""}'


```

<details>
  <summary>Response:</summary>

```yaml
{
    "plain": "hello",
    "cryptic": "mfncz",
    "debug_logs": [
        {
            "idx": 0,
            "offset": [
                "b",
                "a",
                "a"
            ],
            "pass": [
                "[h] - Plugboard ",
                "[h] ↣ Rotor: I",
                "[u] ↣ Rotor: II",
                "[p] ↣ Rotor: III",
                "[q] ⟲ Reflector B",
                "Rotor: III ↢ [y]",
                "Rotor: II ↢ [v]",
                "Rotor: I ↢ [m]",
                "[m] - Plugboard - [m]"
            ]
        },
        {
            "idx": 1,
            "offset": [
                "c",
                "a",
                "a"
            ],
            "pass": [
                "[e] - Plugboard ",
                "[e] ↣ Rotor: I",
                "[b] ↣ Rotor: II",
                "[j] ↣ Rotor: III",
                "[z] ⟲ Reflector B",
                "Rotor: III ↢ [m]",
                "Rotor: II ↢ [o]",
                "Rotor: I ↢ [f]",
                "[f] - Plugboard - [f]"
            ]
        },
        {
            "idx": 2,
            "offset": [
                "d",
                "a",
                "a"
            ],
            "pass": [
                "[l] - Plugboard ",
                "[l] ↣ Rotor: I",
                "[v] ↣ Rotor: II",
                "[y] ↣ Rotor: III",
                "[e] ⟲ Reflector B",
                "Rotor: III ↢ [p]",
                "Rotor: II ↢ [u]",
                "Rotor: I ↢ [n]",
                "[n] - Plugboard - [n]"
            ]
        },
        {
            "idx": 3,
            "offset": [
                "e",
                "a",
                "a"
            ],
            "pass": [
                "[l] - Plugboard ",
                "[l] ↣ Rotor: I",
                "[d] ↣ Rotor: II",
                "[k] ↣ Rotor: III",
                "[j] ⟲ Reflector B",
                "Rotor: III ↢ [e]",
                "Rotor: II ↢ [z]",
                "Rotor: I ↢ [c]",
                "[c] - Plugboard - [c]"
            ]
        },
        {
            "idx": 4,
            "offset": [
                "f",
                "a",
                "a"
            ],
            "pass": [
                "[o] - Plugboard ",
                "[o] ↣ Rotor: I",
                "[k] ↣ Rotor: II",
                "[l] ↣ Rotor: III",
                "[w] ⟲ Reflector B",
                "Rotor: III ↢ [r]",
                "Rotor: II ↢ [g]",
                "Rotor: I ↢ [z]",
                "[z] - Plugboard - [z]"
            ]
        }
    ]
}
```
 </details>


  - Test CORS:

```bash
curl -v --request OPTIONS 'http://127.0.0.1:8000' -H 'Origin: http://aetes.greece.local' -H 'Access-Control-Request-Method: GET'

curl -v --request OPTIONS 'http://enigma_backend:9000' -H 'Origin: http://enigma_frontend:8000' -H 'Access-Control-Request-Method: GET'

curl -v --request OPTIONS 'http://enigma_frontend:8000' -H 'Origin: http://enigma_backend:9000' -H 'Access-Control-Request-Method: GET'

docker exec -it enigma_frontend  curl -v --request OPTIONS 'http://enigma_backend:9000/scrumbles' -H 'Origin: http://localhost:8000' -H 'Access-Control-Request-Method: POST'

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

## Rotors notches specs

```
INPUT		A	B	C	D	E	F	G	H	I	J	K	L	M	N	O	P	Q	R	S	T	U	V	W	X	Y	Z
Rotor I		E	K	M	F	L	G	D	Q	V	Z	N	T	O	W	Y	H	X	U	S	P	A	I	B	R	C	J
Rotor II	A	J	D	K	S	I	R	U	X	B	L	H	W	T	M	C	Q	G	Z	N	P	Y	F	V	O	E
Rotor III	B	D	F	H	J	L	C	P	R	T	X	V	Z	N	Y	E	I	W	G	A	K	M	U	S	Q	O
Rotor IV	E	S	O	V	P	Z	J	A	Y	Q	U	I	R	H	X	L	N	F	T	G	K	D	C	M	W	B
Rotor V		V	Z	B	R	G	I	T	Y	U	P	S	D	N	H	L	X	A	W	M	J	Q	O	F	E	C	K
```


| Rotor         | Notch         | Effect        |
| ------------- | ------------- | ------------- |
|I|Q|If rotor steps from Q to R, the next rotor is advanced|
|II|E|If rotor steps from E to F, the next rotor is advanced|
|III|V|If rotor steps from V to W, the next rotor is advanced|
|IV|J|If rotor steps from J to K, the next rotor is advanced|
|V|Z|If rotor steps from Z to A, the next rotor is advanced|
|VI, VII, VIII|Z+M|If rotor steps from Z to A, or from M to N the next rotor is advanced|

(see wikipedia page, link in footnotes)


<details>

  <summary>Full journey of a letter</summary>

## Enigma I

Configuration (fixed)
Left rotor: III — BDFHJLCPRTXVZNYEIWGAKMUSQO
Middle rotor: II — AJDKSIRUXBLHWTMCQGZNPYFVOE
Right rotor: I — EKMFLGDQVZNTOWYHXUSPAIBRCJ
Reflector: B — YRUHQSLDPXNGOKMIEBFZCWVJAT
Ring settings: A A A
Plugboard: none

### Rotor positions
| Rotor       | Before key | After stepping |
| ----------- | ---------- | -------------- |
| Left (III)  | A          | A              |
| Middle (II) | A          | A              |
| Right (I)   | A          | **B**          |

Letter mappings (A=0 … Z=25)
| Stage       | Letter | Index |
| ----------- | ------ | ----- |
| Key pressed | W      | 22    |

### Forward path (right → left)
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

### Reverse path (left → right)
🔹 Left Rotor — III (reverse wiring)
| Operation     | Value     |
| ------------- | --------- |
| Input         | P (15)    |
| Reverse P → H | **7 (H)** |

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

Final output
| Result    | Letter |
| --------- | ------ |
| Encrypted | **B**  |

</details>

## Acknowledgments & Reference

- [Yew](https://yew.rs/docs/tutorial)
- [Yew examples](https://github.com/yewstack/yew/tree/master/examples)
- [FrancescoXX](https://github.com/FrancescoXX/rust-fullstack-app/blob/main/frontend/src/main.rs)
- [Html interaction web_sys](https://docs.rs/web-sys/latest/web_sys/)

## Enigma:

### Simulators:

- [Cryptii](https://cryptii.com/pipes/enigma-machine)
- [CacheSleuth](https://www.cachesleuth.com/enigma.html)
- [Berlin Physik](https://people.physik.hu-berlin.de/~palloks/js/enigma/enigma-m4_v16_en.html)
- [Cryptool](https://www.cryptool.org/en/cto/enigma/)



### Documentations:

- [Rotors Details](https://en.wikipedia.org/wiki/Enigma_rotor_details)
- [Stanford, the Enigma machine](https://web.stanford.edu/class/cs106j/handouts/36-TheEnigmaMachine.pdf)
- [Code & ciphers #Rotorspec](https://www.codesandciphers.org.uk/enigma/rotorspec.htm)
- [Code & ciphers #ex](https://www.codesandciphers.org.uk/enigma/example1.htm)




> [!NOTE]
> All done mostly to learn & play with Rust... (⌒‿⌒)/


[![Rust](https://github.com/lunarust/enigma/actions/workflows/rust.yml/badge.svg)](https://github.com/lunarust/enigma/actions/workflows/rust.yml)

[![License: WTFPL](https://upload.wikimedia.org/wikipedia/commons/f/fa/WTFPL_badge.png)](/LICENSE.txt)

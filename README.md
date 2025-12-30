# Enigma


## Todo / Could do
- [ ] Issues when my 3rd rotor ticks
- [ ] Add Plugboard
- [ ] Add Option to set rotors start position
- [ ] Remove special characters

## Issue:
with,  
rotor 1, 2, 3
reflector B
Plain text:
DONOTGOGENTLEINTOTHATGOODNIGHTOLDAGESHOULDBURNANDRAVEATCLOSEOFDAYRAGERAGEAGAINSTTHEDYINGOFTHELIGHT

CORRECT >
OLLZLCBDGWWQSOACEKZQAZUDZQSTYLXUNWHLRXMAQGJCSESXJIYNAPMBOZYLUKRSEAUZHFWXCDNNCMPEPUKAFXZJJMVTUCXARE
OLLZLCBDGWWQSOACEKZQAZUDZQSTYLXUNWHLRXMAQGJCSESXJIYNAPMBOZYLUKRSEAUZHFWXCDNNCMPEPUKAFXZJJMVTUCZMYU
< NOT OK

```
Input = I ➞[Rotor 1]➞ S ➞[Rotor 2]➞ B ➞[Rotor 3]➞ D ➞[Reflector]➞ H ➞[Rotor 3]➞ D ➞[Rotor 2]➞ H ➞[Rotor 1]➞ X = Output

94 REB [I] ↣ I Enigma I [S] ↣ II Enigma I [B] ↣ III Enigma I [E] ⟲ Reflector B [E] ↢ III Enigma I [H] ↢ II Enigma I [G] ↢ I Enigma I ↢ Z
95 SEB [G] ↣ I Enigma I [K] ↣ II Enigma I [I] ↣ III Enigma I [S] ⟲ Reflector B [S] ↢ III Enigma I [R] ↢ II Enigma I [T] ↢ I Enigma I ↢ M
96 TEB [H] ↣ I Enigma I [L] ↣ II Enigma I [Y] ↣ III Enigma I [N] ⟲ Reflector B [N] ↢ III Enigma I [E] ↢ II Enigma I [B] ↢ I Enigma I ↢ Y
97 UEB [T] ↣ I Enigma I [C] ↣ II Enigma I [N] ↣ III Enigma I [X] ⟲ Reflector B [X] ↢ III Enigma I [T] ↢ II Enigma I [E] ↢ I Enigma I ↢ U

```



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

curl --location 'localhost:9000/scrumble' \
--header 'Content-Type: application/json' \
--data '{"cryptic":"","plain":"welcome","reflector":{"definition":"YRUHQSLDPXNGOKMIEBFZCWVJAT","id":4,"model":"M3","name":"Reflector B"},"rotor":[{"definition":"EKMFLGDQVZNTOWYHXUSPAIBRCJ","id":0,"model":"Enigma I","name":"I Enigma I","notch":["Q"]},{"definition":"AJDKSIRUXBLHWTMCQGZNPYFVOE","id":1,"model":"Enigma I","name":"II Enigma I","notch":["E"]},{"definition":"BDFHJLCPRTXVZNYEIWGAKMUSQO","id":2,"model":"Enigma I","name":"III Enigma I","notch":["V"]}]}'

```

<details>
  <summary>Response:</summary>

```yaml
{
    "plain": "welcome",
    "cryptic": "BFNLZKH",
    "debug_logs": [
        {
            "idx": 0,
            "offset": [
                "B",
                "A",
                "A"
            ],
            "pass": [
                "[w] ↣ I Enigma I",
                "[Q] ↣ II Enigma I",
                "[Q] ↣ III Enigma I",
                "[I] ⟲ Reflector B",
                "[I] ↢ III Enigma I",
                "[H] ↢ II Enigma I",
                "[L] ↢ I Enigma I ↢ B"
            ]
        },
        {
            "idx": 1,
            "offset": [
                "C",
                "A",
                "A"
            ],
            "pass": [
                "[e] ↣ I Enigma I",
                "[B] ↣ II Enigma I",
                "[J] ↣ III Enigma I",
                "[T] ⟲ Reflector B",
                "[T] ↢ III Enigma I",
                "[M] ↢ II Enigma I",
                "[O] ↢ I Enigma I ↢ F"
            ]
        },
        {
            "idx": 2,
            "offset": [
                "D",
                "A",
                "A"
            ],
            "pass": [
                "[l] ↣ I Enigma I",
                "[V] ↣ II Enigma I",
                "[Y] ↣ III Enigma I",
                "[Q] ⟲ Reflector B",
                "[Q] ↢ III Enigma I",
                "[P] ↢ II Enigma I",
                "[U] ↢ I Enigma I ↢ N"
            ]
        },
        {
            "idx": 3,
            "offset": [
                "E",
                "A",
                "A"
            ],
            "pass": [
                "[c] ↣ I Enigma I",
                "[Z] ↣ II Enigma I",
                "[E] ↣ III Enigma I",
                "[J] ⟲ Reflector B",
                "[J] ↢ III Enigma I",
                "[K] ↢ II Enigma I",
                "[D] ↢ I Enigma I ↢ L"
            ]
        },
        {
            "idx": 4,
            "offset": [
                "F",
                "A",
                "A"
            ],
            "pass": [
                "[o] ↣ I Enigma I",
                "[K] ↣ II Enigma I",
                "[L] ↣ III Enigma I",
                "[V] ⟲ Reflector B",
                "[V] ↢ III Enigma I",
                "[R] ↢ II Enigma I",
                "[G] ↢ I Enigma I ↢ Z"
            ]
        },
        {
            "idx": 5,
            "offset": [
                "G",
                "A",
                "A"
            ],
            "pass": [
                "[m] ↣ I Enigma I",
                "[M] ↣ II Enigma I",
                "[W] ↣ III Enigma I",
                "[U] ⟲ Reflector B",
                "[U] ↢ III Enigma I",
                "[G] ↢ II Enigma I",
                "[R] ↢ I Enigma I ↢ K"
            ]
        },
        {
            "idx": 6,
            "offset": [
                "H",
                "A",
                "A"
            ],
            "pass": [
                "[e] ↣ I Enigma I",
                "[M] ↣ II Enigma I",
                "[W] ↣ III Enigma I",
                "[U] ⟲ Reflector B",
                "[U] ↢ III Enigma I",
                "[G] ↢ II Enigma I",
                "[R] ↢ I Enigma I ↢ H"
            ]
        }
    ]
}
```
 </details>
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

## Rotors notches specs (see wikipedia)

| Rotor         | Notch         | Effect        |
| ------------- | ------------- | ------------- |
|I|Q|If rotor steps from Q to R, the next rotor is advanced|
|II|E|If rotor steps from E to F, the next rotor is advanced|
|III|V|If rotor steps from V to W, the next rotor is advanced|
|IV|J|If rotor steps from J to K, the next rotor is advanced|
|V|Z|If rotor steps from Z to A, the next rotor is advanced|
|VI, VII, VIII|Z+M|If rotor steps from Z to A, or from M to N the next rotor is advanced|



<details>

  <summary>Full journey of a letter</summary>

### Enigma I

Configuration (fixed)
Left rotor: III — BDFHJLCPRTXVZNYEIWGAKMUSQO
Middle rotor: II — AJDKSIRUXBLHWTMCQGZNPYFVOE
Right rotor: I — EKMFLGDQVZNTOWYHXUSPAIBRCJ
Reflector: B — YRUHQSLDPXNGOKMIEBFZCWVJAT
Ring settings: A A A
Plugboard: none

#### Rotor positions
| Rotor       | Before key | After stepping |
| ----------- | ---------- | -------------- |
| Left (III)  | A          | A              |
| Middle (II) | A          | A              |
| Right (I)   | A          | **B**          |

Letter mappings (A=0 … Z=25)
| Stage       | Letter | Index |
| ----------- | ------ | ----- |
| Key pressed | W      | 22    |

#### Forward path (right → left)
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

#### Reverse path (left → right)
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

[Yew](https://yew.rs/docs/tutorial)

[Yew examples](https://github.com/yewstack/yew/tree/master/examples)

[FrancescoXX](https://github.com/FrancescoXX/rust-fullstack-app/blob/main/frontend/src/main.rs)

[Html interaction web_sys](https://docs.rs/web-sys/latest/web_sys/)

## Enigma:

### Simulators:

[Cryptii](https://cryptii.com/pipes/enigma-machine)

[CacheSleuth](https://www.cachesleuth.com/enigma.html)

[Berling Physik](https://people.physik.hu-berlin.de/~palloks/js/enigma/enigma-m4_v16_en.html)

[Cryptool](https://www.cryptool.org/en/cto/enigma/)



### Documentations:

[Rotors Details](https://en.wikipedia.org/wiki/Enigma_rotor_details)



> [!NOTE]
> All done mostly to learn & play with Rust... (⌒‿⌒)/



[![License: WTFPL](https://upload.wikimedia.org/wikipedia/commons/f/fa/WTFPL_badge.png)](/LICENSE.txt)

# Endpoints
## POST `/send_code`

Request

```json
{
   "phone_number": "phone number"  
}
```

Response

```json
{}
```

## POST `/verify_code`

Note: Use same session with `send_code`.

Request

```json
{
   "code": "One time password (sent over SMS)"  
}
```

Response
```json
{
   "token": "long-term token"
}
```

## POST  `/issue_credential`

Request

```json
{
    "token": "long-term token",
    "openers": [
        "Opener1's URL",
        "Opener2's URL",
        "Opener3's URL"
    ]
}
```

Response

```json
{
    "xi": "anonymous authentication credential 1",
    "ai": "anonymous authentication credential 2",
    "gamma": "anonymous authentication credential 3",
}
```
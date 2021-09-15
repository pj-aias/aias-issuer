# aias-issuer

Issuer entity of AIAS project.  
This entity is responsible for first registering for users.  

**Note: In this repository, the server use sms authentication, but we should use eKYC.**

## How to install
### 1. Clone Repository

Clone this repository.

```sh
git clone https://github.com/pj-aias/aias-issuer
cd aias-issuer
```

### 2. Set enviroment variables

Write .env file to set enviroment variables.
It have to fill all variable to work.

```sh
cp .env.sample .env
vim .env # write
```

```sh
AIAS_DEBUG=false
AIAS_ISSUER_PRIVKEY="RSA private key PEM file"
SMS_FROM="twilio phone number"
ACCOUNT_ID="twilio account id"
AUTH_TOKEN="twilio authentication token"
```

### 3. Crate database

Create sqlite3 DB with the cat command.

```sh
touch aias.db
```

### 4. Run servers

Run servers with docker-compose command.

```sh
docker-compose up 
# cargo run
```


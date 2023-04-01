## RustiClipboard
Web service which allows the user to paste and share clipboard content (clips)

### Features:
* Link sharing
* Clip expiration
* JSON API
* Password protected clips

### Technical details
* Asynchronous web server
* Template rendering engine
* CLI API client that will include an API key generation and revocation
* SQLite DB with migrations and deferred database writes for maximum performance
* Background service which performs routine clean-up tasks

The entire project is designed using multilayered enterprise architecture which is fully testable and allows you to easily manage code as it grows in size. 

### Setup sqlite for sqlx
```
export DATABASE_URL="sqlite:data.db"
```
```
sqlx db create && sqlx migrate run
```
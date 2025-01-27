# vape-db

`vape-db` is a temporary in-memory database with absolutely no persistence.
Its use is to provide communication between different parts of any system (for example to enable communication between two Jenkins builds).

Consult the `API.md` file for more information on how to use the API.

## Release Notes

### 0.2.0

- Limiting the number of entries in the database
    - The oldest entries will be removed when the limit is reached
- Adding endpoints to get info about when the entry was created and last updated
- Adding endpoints to get all entries with a certain prefix
- New entries:
    - `GET /all_entries/{id_prefix}`
    - `DELETE /all_entries/{id_prefix}`
    - `GET /infos/{*id}`
    - `GET /all_infos/{id_prefix}`
- The `/status` endpoint now reports the version of the database

### 0.1.0

- Initial release
    - Basic in-memory database, no limits
- Endpoints:
    - `GET /entries/{*id}`
    - `POST /entries/{*id}`
    - `DELETE /entries/{*id}`
    - `GET /status`
    - `GET /health`

# vape-db API Documentation

## Overview

The vape-db API allows users to interact with the vape-db system to store and retrieve data. This documentation provides details on how to use the API from a user's perspective.

## General Information

- **Content Type**: `application/json`
- **Authentication**: API key required in the header

## Limitations

- **Storage Limit**: There is a limit on how many elements can be stored in the vape-db.
- **Data Persistence**: Users should expect the database to be empty at any point, as data may be cleared periodically.

## Endpoints

### 1. Store Data

**Endpoint**: `POST /store`

**Description**: Stores data in the vape-db.

**Request Body**:
```json
{
  "key": "string",
  "value": "string"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Data stored successfully"
}
```

### 2. Retrieve Data

**Endpoint**: `GET /retrieve`

**Description**: Retrieves data from the vape-db.

**Query Parameters**:
- `key` (required): The key of the data to retrieve.

**Response**:
```json
{
  "key": "string",
  "value": "string"
}
```

### 3. Delete Data

**Endpoint**: `DELETE /delete`

**Description**: Deletes data from the vape-db.

**Request Body**:
```json
{
  "key": "string"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Data deleted successfully"
}
```

### 4. List All Keys

**Endpoint**: `GET /keys`

**Description**: Lists all keys currently stored in the vape-db.

**Response**:
```json
{
  "keys": ["string"]
}
```

## Error Handling

In case of errors, the API will return a JSON object with the following fields:

```json
{
  "status": "error",
  "message": "Error message describing what went wrong"
}
```

## Example Usage

### Store Data Example

**Request**:
```http
POST /store
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY

{
  "key": "exampleKey",
  "value": "exampleValue"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Data stored successfully"
}
```

### Retrieve Data Example

**Request**:
```http
GET /retrieve?key=exampleKey
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY
```

**Response**:
```json
{
  "key": "exampleKey",
  "value": "exampleValue"
}
```

### Delete Data Example

**Request**:
```http
DELETE /delete
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY

{
  "key": "exampleKey"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Data deleted successfully"
}
```

### List All Keys Example

**Request**:
```http
GET /keys
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY
```

**Response**:
```json
{
  "keys": ["exampleKey1", "exampleKey2"]
}
```
# Operations
*Authentication is currently not required. No headers are necessary. All request and (success) response bodies are formatted as valid JSON objects. Query strings are formatted as an ampersand-seperated list of `key=value` pairs. Request bodies and query strings should be completely omitted when not required. The server may or may not accept unrequested body data and/or query parameters without comment.*

*All operations currently respond with an HTTP status of 200 for success or 4xx-5xx for failure. Error formats are currently undocumented.*
*CORS OPTIONS requests are fully supported.*

## Define DB (`/admin/define`)
Creates the database schema.

### Description
Should only be called once to initialize the database. Requires the D1 resource to already exist. 

### Initialize
Request:
- Method - `POST`
- Path - `/admin/define`

Response:  
`<success message>`

## User (`/user`)
Accesses user data.

### Description
The `/user` endpoint allows reading user data or creating a new user.

### Read
Request:
- Method - `GET`
- Query
  - `id` = `<the requested user's id>`

Response:
```
{
  "username": <String - the user's username>,
  "password": <String - the user's password>,
  "role": <String - the user's role>
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "username": <String - the user's username>,
    "password": <String - the user's password>,
    "role": <String - the user's role>
  }
  ```

Response:
`<success message>`

## Faculty (`/faculty`)
Accesses faculty data.

### Description
The `/faculty` endpoint allows reading faculty data or creating a new faculty member.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested faculty member's id>`

Response:
```
{
  "name": <String - the member's name>,
  "email": <String - the member's email address>,
  "department": <String - the member's department>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "name": <String - the member's name>,
        "email": <String - the member's email address>,
        "department": <String - the member's department>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "name": <String - the member's name>,
    "email": <String - the member's email address>,
    "department": <String - the member's department>
  }
  ```

Response:
`<success message>`

## Class (`/class`)
Accesses class data.

### Description
The `/class` endpoint allows reading class data or creating a new class.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested class's id>`

Response:
```
{
  "name": <String - the class's name>,
  "description": <String - the class's description>,
  "capacity": <Int - the class's capacity>,
  "code": <String - the class's code>,
	"kind": <String - the class's type>,
	"section": <String - the class's section>,
	"term": <String - the class's term>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "name": <String - the class's name>,
        "description": <String - the class's description>,
        "capacity": <Int - the class's capacity>,
        "code": <String - the class's code>,
        "kind": <String - the class's type>,
        "section": <String - the class's section>,
        "term": <String - the class's term>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "name": <String - the class's name>,
    "description": <String - the class's description>,
    "capacity": <Int - the class's capacity>,
    "code": <String - the class's code>,
    "kind": <String - the class's type>,
    "section": <String - the class's section>,
    "term": <String - the class's term>
  }
  ```

Response:
`<success message>`

## Room (`/room`)
Accesses room data.

### Description
The `/room` endpoint allows reading room data or creating a new room.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested room's id>`

Response:
```
{
  "number": <String - the room's number>,
  "capacity": <Int - the room's capacity>,
  "kind": <String - the room's type>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "number": <String - the room's number>,
        "capacity": <Int - the room's capacity>,
        "kind": <String - the room's type>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "number": <String - the room's number>,
    "capacity": <Int - the room's capacity>,
    "kind": <String - the room's type>
  }
  ```

Response:
`<success message>`

## Schedule (`/schedule`)
Accesses schedule data.

### Description
The `/schedule` endpoint allows reading schedule data or creating a new schedule.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested schedule's id>`

Response:
```
{
  "start": {
    "hour": <Int - the schedule's start hour>,
    "minute": <Int - the schedule's start minute>
  },
   "end": {
    "hour": <Int - the schedule's end hour>,
    "minute": <Int - the schedule's end minute>
  },
  "days": <String - the schedule's days>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "start": {
          "hour": <Int - the schedule's start hour>,
          "minute": <Int - the schedule's start minute>
        },
        "end": {
          "hour": <Int - the schedule's end hour>,
          "minute": <Int - the schedule's end minute>
        },
        "days": <String - the schedule's days>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "start": {
      "hour": <Int - the schedule's start hour>,
      "minute": <Int - the schedule's start minute>
    },
    "end": {
      "hour": <Int - the schedule's end hour>,
      "minute": <Int - the schedule's end minute>
    },
    "days": <String - the schedule's days>
  }
  ```

Response:
`<success message>`

## Feature (`/feature`)
Accesses feature data.

### Description
The `/feature` endpoint allows reading feature data or creating a new feature.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested feature's id>`

Response:
```
{
  "name": <String - the features's name>,
  "description": <String - the feature's description>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "name": <String - the features's name>,
        "description": <String - the feature's description>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "name": <String - the features's name>,
    "description": <String - the feature's description>
  }
  ```

Response:
`<success message>`

## Preference (`/pref`)
Accesses preference data.

### Description
The `/pref` endpoint allows reading preference data or creating a new preference.

### Read Single
Request:
- Method - `GET`
- Query
  - `scope` = `single` *(optional)*
  - `id` = `<the requested preference's id>`

Response:
```
{
  "faculty": <Int - the faculty id for the preference>,
  "kind": <String - the preference's type>,
  "value": <String - the preference's value>
}
```

### Read All
Request:
- Method - `GET`
- Query
  - `scope` = `all`

Response:
```
{
  "entries": [
    {
      "id": <Int - the entry's db id>,
      "data": {
        "faculty": <Int - the faculty id for the preference>,
        "kind": <String - the preference's type>,
        "value": <String - the preference's value>
      }
    },
    ...
  ]
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "faculty": <Int - the faculty id for the preference>,
    "kind": <String - the preference's type>,
    "value": <String - the preference's value>
  }
  ```

Response:
`<success message>`

## Class Schedule Room (`/csr`)
Accesses CSR data.

### Description
The `/csr` endpoint allows reading csr data or creating a new csr.

### Read
Request:
- Method - `GET`
- Query
  - `id` = `<the requested csr's id>`

Response:
```
{
  "class": <Int - the class id for the csr>,
  "room": <Int - the room id for the csr,
  "schedule": <Int - the schedule id for the csr>
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "class": <Int - the class id for the csr>,
    "room": <Int - the room id for the csr,
    "schedule": <Int - the schedule id for the csr>
  }
  ```

Response:
`<success message>`

### Update
Request:
- Method - `PUT`
- Query
  - `id` = `<the id of the csr to update>`
- Body  
  ```
  {
    "column": <String - the csr column name to update>,
    "value": <String - the value for the updated column>
  }
  ```

Response:
`<success message>`

## Class Faculty (`/cf`)
Accesses CF data.

### Description
The `/cf` endpoint allows reading cf data or creating a new cf.

### Read
Request:
- Method - `GET`
- Query
  - `id` = `<the requested cf's id>`

Response:
```
{
  "class": <Int - the class id for the cf>,
  "faculty": <Int - the faculty id for the cf>
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "class": <Int - the class id for the cf>,
    "faculty": <Int - the faculty id for the cf>
  }
  ```

Response:
`<success message>`

## Room Feature (`/rf`)
Accesses RF data.

### Description
The `/rf` endpoint allows reading rf data or creating a new rf.

### Read
Request:
- Method - `GET`
- Query
  - `id` = `<the requested rf's id>`

Response:
```
{
  "room": <Int - the room id for the rf>,
  "feature": <Int - the feature id for the rf>
}
```

### Create
Request:
- Method - `PUSH`
- Body  
  ```
  {
    "room": <Int - the room id for the rf>,
    "feature": <Int - the feature id for the rf>
  }
  ```

Response:
`<success message>`

## Notes
- **Table Validation**:  
Before executing the queries with table names, the `delete_by_id` function verifies that the `table_name` is valid. This helps prevent SQL injection attacks by ensuring that the table name matches a known, safe, and hard-coded list of table names.  
*NOTE: is this still valid?*
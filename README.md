# GraphQL Schema

Explore a GraphQL Schema given an endpoint URL.

## Usage
```shell
graphql-explorer 0.1.0
Explore GraphQL schema given a URL

USAGE:
    graphql-schema-rs <schema-url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <schema-url>    URL to a GraphQL API Endpoint
```

```shell
> graphql-schema https://bahnql.herokuapp.com/graphql
+----------------------+--------+--------------------------------------------------------------+
| NAME                 | KIND   | DESCRIPTION                                                  |
+----------------------+--------+--------------------------------------------------------------+
| Query                | OBJECT |                                                              |
+----------------------+--------+--------------------------------------------------------------+
| Int                  | SCALAR | The `Int` scalar type represents non-fractional signed whole |
|                      |        | numeric values. Int can represent values between -(2^31) and |
|                      |        | 2^31 - 1.                                                    |
+----------------------+--------+--------------------------------------------------------------+
| Route                | OBJECT |                                                              |
+----------------------+--------+--------------------------------------------------------------+
| RoutePart            | OBJECT |                                                              |
+----------------------+--------+--------------------------------------------------------------+
| Station              | OBJECT |                                                              |
+----------------------+--------+--------------------------------------------------------------+
| String               | SCALAR | The `String` scalar type represents textual data,            |
|                      |        | represented as UTF-8 character sequences. The String type is |
|                      |        | most often used by GraphQL to represent free-form human-     |
|                      |        | readable text.                                               |
+----------------------+--------+--------------------------------------------------------------+
| Location             | OBJECT |                                                              |
+----------------------+--------+--------------------------------------------------------------+
| Float                | SCALAR | The `Float` scalar type represents signed double-precision   |
|                      |        | fractional values as specified by [IEEE                      |
|                      |        | 754](http://en.wikipedia.org/wiki/IEEE_floating_point).      |
+----------------------+--------+--------------------------------------------------------------+
| Boolean              | SCALAR | The `Boolean` scalar type represents `true` or `false`.      |
+----------------------+--------+--------------------------------------------------------------+
```

## Future Considerations
Several features to consider:
- Allow for query (`--query`) and schema (`--schema`) path inputs
- Parse nested fields in FullType (in `query.graphql`)
- Allow for `authorization` (`--authorization`) for when using an enpoint that requires authorization
- Allow for writing to file output (`--output`)
- Allow user to toggle pretty vs json (`--pretty`)
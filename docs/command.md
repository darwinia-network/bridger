## Command

Here are some documents of common commands

### kv

The darwinia-bridger will save some data to the embedded kv database. `kv` command can be read or write database.

```bash
$ bridger kv
bridger-kv 0.4.5
The bridge kv db storage operation

USAGE:
    bridger kv [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --namespace <namespace>    The namespace of storage
        --server <server>          The server host by bridger service [default: http://127.0.0.1:1098]

SUBCOMMANDS:
    get           Get Key-Value from bridger
    help          Prints this message or the help of the given subcommand(s)
    list          List bridger database
    namespaces    Show all namespaces
    put           Put Key-Value to bridger database
    remove        Remove a Key-Value from bridger
```

#### namespaces

Show all namespaces.

```bash
$ bridger kv namespaces
<DEFAULT>
aoo
boo
```

If the namespace is not set, the default namespace is an empty string, is shown as `<DEFAULT>`

#### put

put to default namespace

```bash
$ bridger kv put foo bar
```

put to custom namespace
```bash
$ bridger kv --namespace boo put foo bar
````

put single key-value

```bash
$ bridger kv --namespace aoo put foo0 bar0
```

put multiple key-value

```bash
$ bridger kv --namespace aoo put foo1 bar1 foo2 bar2
```

put with custom data type

```bash
$ bridger kv --namespace aoo put foo3::u16 12345 is_test::bool true
```

support types

- `String` `string` `str`
- `isize` `i8` `i16` `i32` `i64` `i128`
- `usize` `u8` `u16` `u32` `u64` `u128`
- `f32` `f64`
- `bool`

#### list

list of default namespace
```bash
$ bridger kv list
foo
```

list of custom namespace

```bash
$ bridger kv -n aoo list
foo0
foo1
foo2
foo3
is_test
```

```bash
$ bridger kv -n boo list
foo
```

#### get

get value of single key

```bash
$ bridger kv get foo
bar
```

get multiple keys

```bash
$ bridger kv -n aoo get foo1 foo2 foo3
bar1
bar2
12345
```

the result is include key and custom output format

```bash
$ bridger kv -n aoo get foo1 foo2 foo3 --output json --include-key
{
  "foo2": "bar2",
  "foo3": 12345,
  "foo1": "bar1"
}
```

```bash
$ bridger kv -n aoo get foo1 foo2 foo3 --output table --include-key
+------+-------+
| foo1 | bar1  |
+------+-------+
| foo2 | bar2  |
+------+-------+
| foo3 | 12345 |
+------+-------+
```

#### remove

Remove key

```bash
$ bridger kv remove foo
Success
```

Remove key with custom namespace

```bash
$ bridger kv -n aoo remove foo1
Success
```


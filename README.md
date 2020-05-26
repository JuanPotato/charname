# charname
A library to get the name of unicode characters. [Documentation](https://doc.rs/charname)

## Usage
There are two available functions `get_name` and `get_name_checked`.

`get_name` will always return a `&str`. If a certain character is unknown, it returns `UNKNOWN CHARACTER`

`get_name_checked` returns `Option<&str>`. If a certain character is unknown, it returns `None`

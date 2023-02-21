# Teratemplar

Teratemplar is a simple cli tool for rendering tera templates with structured text as input.

Currently supported input format are:
toml

---
## Example
template.tera
```
Hello {{ user.name }}!
```

input.toml
```toml
[user]
name = "world"
```

Running the following renders the input data into the template
```
teratemplar -i input.toml -f toml -t template.tera -o outfile
```
```
Hello world!
```
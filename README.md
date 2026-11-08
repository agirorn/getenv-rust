# getenv

Print out an environment variable value after loading environment variables from
a .env file, if available, and mashes those with the environment variables
provided by the operative system

## Usage

```shell
echo "VARIABLE_NAME=THE_VALUE" >> .env
getenv VARIABLE_NAME
getenv -i variable_name
```

## Build

```shell
make Build
```

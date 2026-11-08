# getenv

Print out an environment variable value after loading environment variables from
a .env file, if available, and mashes those with the environment variables
provided by the operative system

## Usage

```shell
echo "VARIABLE_NAME=THE_VALUE" >> .env
getenv VARIABLE_NAME
THE_VALUE # Print out
getenv -i variable_name
THE_VALUE # Print out
getenv BAD_VARIABLE_NAME
Environment variable (BAD_VARIABLE_NAME) not found # Print out
echo $?
1 # Print out
```

## Build

```shell
make Build
```

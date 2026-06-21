# CrabbyFig

CrabbyFig is a small tool to help process environment variable based configuration. It is intended to be used as part of
the startup process in Docker containers.

Fully supports passing secrets via `*_FILE` vars.

## Configure Your Environment

### (REQUIRED) Setting String Replacements

In the files you need to configure, add replacement strings for each value you want to replace.

Use the format: `REPLACE_< CRABBYFIX>_< whatever you want >`.

In the process environment, set each replacement string using a key that matches the format:
`< CRABBYFIX>_< whatever you want >`.

It should match the string to be replaced, jut without the `REPLACE_` prefix.

So, `REPLACE_MYPREFIX_MYDATAITEM` would be replaced by the value in the variable `MYPREFIX_MYDATAITEM`.

### (OPTIONAL) Defaults File

For each prefix, you can include a "DEFAULTS" file.

Just set an environment variable using the format: `< CRABBYFIX>_DEFAULTS_FILE`.

For example: `MYPREFIX_DEFAULTS_FILE=/path/to/file`, then in `/path/to/file` you can set the default value for
`MYPREFIX_MYDATAITEM`.

One variable per line.

When loading, the defaults file is loaded first so that any environment variables will override the defaults.

### (REQUIRED) Set `CRABBYFIX`

The value should be the prefix you are using for your service environment variables.

Must end in an underscore. `MYPREFIX_`.

More than one prefix is supported. List them separated via commas. `MYPREFIX_,ANOTHERPREFIX_`

#### (REQUIRED) Set `CRABBYGETS`

Lists the absolute paths to the files that should be processed.

Separated via commas: `/path/to/a,/path/to/b`

#### (OPTIONAL) Set `CRABBYGETS_FILE`

The absolute path of a file that contains a list of files that should be processed.

This is meant for when the normal `CRABBYGETS` var grows too large.

Either keep the file as a single CSV line, or put one file per line.

CrabbyFig will convert the file into a single CSV line, and then add it to the main `CRABBYGETS` string.

4) In your config files, the replacement strings should follow this pattern:
   `REPLACE_< CRABBYFIX>_< whatever you want >`. Then, in your service environment, you would set the variable names to
   be `< CRABBYFIX >_< whatever you want >`. If the variable name ends in `_FILE`, CrabbyFig will treat it as a file
   path and load the replacement string from that file.
5) Finally, make sure to run CrabbyFig before the processes you are configuring start. For example, if you use
   s6-overlay, add CrabbyFig as oneshot that your other services depend on.

## Example

`/etc/apt/sources.list.d/myrepo.sources`:

```
Types: deb
URIs: REPLACE_EXAMPLEAPP_PRIMARY_URI REPLACE_EXAMPLEAPP_SECONDARY_URI
Suites: REPLACE_EXAMPLEAPP_SUITES
Components: REPLACE_EXAMPLEAPP_COMPONENTS
Signed-By: REPLACE_EXAMPLEAPP_SIGNED_BY
```

`docker-compose.yml`:

```yaml
secrets:
  example_signed_by:
    file: ./my_secret_path

services:
  exampleapp:
    image: exampleapp:latest
    environment:
      CRABBYFIX: EXAMPLEAPP
      CRABBYGETS: /etc/apt/sources.list.d/myrepo.sources
      EXAMPLEAPP_SIGNED_BY_FILE: /run/secrets/example_signed_by
      EXAMPLEAPP_PRIMARY_URI: "https://example.com/repo/"
      EXAMPLEAPP_SECONDARY_URI: "https://example.com/repo/secondary"
      EXAMPLEAPP_SUITES: "stable"
      EXAMPLEAPP_COMPONENTS: "main"
```

## Security

This tool is only as secure as your environment is secure. If an environment variable is changed before CrabbyFig runs,
then CrabbyFig will use that value.

It needs to run with enough permissions to modify the files you want modified.

So, do your due diligence and check that the code you are running is safe.

If you find a security issue, please email david+security@reagannetworks.com.

## Development

CrabbyFig is meant to be as simple as possible. So beyond fixing any issues I run into as I implement it, I don't
foresee needing to add any new features.

### Automated Testing

The one area I would like to improve is automated testing. Because the entire point of CrabbyFig is to work with
environment variables, implementing tests normally can only go so far. Because environment variables are outside the
process, when cargo is running tests multi-threaded (the default) you run into issues with keeping the environment
variables consistent.

## AI

I consulted JetBrains AI Assistant while developing CrabbyFig. I did not just have it develop CrabbyFig for me. I do not
trust AI code unless I can fully understand it. So, while some code was copy and pasted, it was only after I made sure I
understood it.


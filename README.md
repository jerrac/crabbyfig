# CrabbyFig

CrabbyFig is a small tool to help process environment variable based configuration. It is intended to be used as part of
the startup process in Docker containers.

Fully supports passing secrets via `*_FILE` vars.

## Usage

1) Set `CRABBYFIX` to the prefix you are using for your service environment variables.
2) Set `CRABBYGETS` to a comma-separated list of files to replace strings in.
3) (Optional) Set `CRABBYGETS_FILE` to the path of a file in the same environment as you are running CrabbyFig that
   contains a list of files to replace strings in. This is meant for when the normal `CRABBYGETS` var grows too large.
   Either keep the file as a single CSV line, or put one file per line. CrabbyFig will convert the file into a single
   CSV line, and then add it to the main `CRABBYGETS` string.
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


git-who
=================

List remote branches by author and date of last commit
 - Coloured dates highlight age

# Usage

```shell
$ git-who --help
git-who
List remote branches by author and date of last commit

USAGE:
    git-who [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --remote <NAME>    Remote name [default: origin]
```

# Example

```shell
$ git-who
[2016-06-10] James Kirk    shore-leave-locations
[2016-06-17] James Kirk    harry-mudd-emails
[2016-05-24] Leonard McCoy reasons-spock-not-fit-for-command
[2016-06-21] Leonard McCoy dead-officers-list
[2016-06-21] Leonard McCoy doctor-not-a
[2015-03-11] Spock         talos-iv-return-plan
[2015-09-16] Spock         3d-chess-notes
[2015-10-12] Spock         hands-on-reactor-repair-guide
```

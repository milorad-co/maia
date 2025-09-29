# Milorad Automated Installation Application
MAIA is commandline installation application with a similar syntax to Apt. It is designed to install applications from GitHub, the world's largest open-source file sharing website. It can download executable files from the releases of any github repository*!<br>
*Excluding repositories in the [Limitations section](https://github.com/milorad-co/maia#Limitations).
# How to Install
Note that MAIA is compatible only with Debian-based GNU/Linux.
1. Install cURL with `sudo apt install curl`.
2. Download MAIA from one of our releases.
3. Execute it from the commandline (if you are trying to use a commandline installation application we would hope that you know how to do that) *as root* with the argument `setup` (e.g. `sudo ./maia setup`). It will then install itself into your `/usr/bin` directory and allow you to use it as if it were a command.
# How to Use
As mentioned earlier, MAIA has a syntax very similar to Apt's syntax. **There are differences**, however, and some may simply not use Apt, so we have added this documentation. (Note that you can access similar documentation via the application by running `maia help`).
## Syntax
Syntax: `maia <subcommand> <arguments>`<br>
## Subcommands
`install`     install an application (requires root priveleges)<br>
`remove`      remove an application (requires root priveleges)<br>
`override`    same as install, but does not throw an error if it has to overwrite files, can be used to update applications (requires root priveleges)<br>
`help`        show help information, or show more detailed information about a specific subcommand
### install and override
Syntax: `sudo maia <install or override> <account name>/<repository name>`<br>
The install subcommand is used to install applications. To specify the application to install, you must enter the name of the account which owns the repository, followed by a forward slash (/), followed by the name of the repository. For example, you could run `sudo maia install milorad-co/mica` to install MICA, our image editor, or you could run `sudo maia install fish-shell/fish-shell` to install Fish, the Friendly Interactive Shell. Please note that to install an application, the targeted repository must have at least one release and its latest release must have at least one asset that is not source code. The install subcommand will fail if it tries to overwrite files.<br>
The override subcommand is similar to install, except it will not fail if forced to overwrite files. It can be used to update applications, as it will install the latest release and overwrite outdated files.
### remove
Syntax: `sudo maia remove <account name>/<repository name>`<br>
Remove removes applications and any configuration data that MAIA created for them. Note that some data may remain, such as extremely out of date files whose references were removed during an override.
# Limitations
1. MAIA cannot extract archives. We are going to fix this soon, but for now, all repositories with these files are not installable.
2. MAIA requires for the repository to have at least one release.
3. The repository must have a release marked as latest.
4. The latest release must have at least one non-source-code asset.
# Compilation from Source
```
git clone https://github.com/milorad-co/maia
cd maia
cargo build
```
Then the file will be in the `maia/target/debug` directory.
# Issues
MAIA has some trouble removing certain applications that were installed with the archive extraction tools.

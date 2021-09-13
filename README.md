# Parameter Store Executor
Fetches parameters recursively at PARAMETER_PATHs from AWS SSM Parameter Store.  
Then executes CMD with the parameters transformed into ENV variables.

The parameter names will be transformed as:
 - Make relative to the corresponding PARAMETER_PATH
 - Replace all '/' & '-' characters with '_'
 - Make UPPERCASE

Conflicting parameters will resolve to the value of the last one found.
Any existing ENV variables (unless --clean-env is specified) will be passed
along and takes precedence over parameters with the same name - to allow
overriding specific parameters (e.g in development environment).

```gherkin
Given the following parameters:
| name      | value |
| /one/test | 1     |
| /two/test | 2     |

When requesting: [/, /one, /two]

Then the following ENV variables will be available:
| name     | value |
| ONE_TEST | 1     |
| TWO_TEST | 2     |
| TEST     | 2     |
```

## Installation

### Build from source
1. Clone the repo
2. Run `shards build`
3. Copy the executable found at `./bin/pse` to a location in your `$PATH`


### Released binary
Download the desired version from the [releases](https://github.com/neochrome/parameter-store-executor/releases) page.


## Usage
When started, the tool will try to detect your current AWS credentials in the following order:
1. From [ENV variables](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html)
2. From [`~/.aws/credentials`](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
3. From the [instance metadata service](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html) (useful on EC2 instances)

### Basic command line usage
```sh
pse /path/to/parameters -- env
```
Use the `--help` option for further information on how to invoke the tool.

### With Docker

#### Install the binary release from GitHub
```Dockerfile
FROM alpine

# install a specific binary release
ARG pse_version=latest
ADD https://github.com/neochrome/parameter-store-executor/releases/download/${pse_version}/pse /
# -- or --
# use the latest released version
ADD https://github.com/neochrome/parameter-store-executor/releases/latest/download/pse /
```

#### Install the binary release from Docker Hub
The binary release is additionally pushed to Docker Hub and may be installed using a
multi-stage docker build like so:

```Dockerfile
# install a specific binary release
FROM neochrome/parameter-store-executor:0.2.0 as release
# -- or --
# use the latest version
FROM neochrome/parameter-store-executor:latest as release

COPY --from=release /pse /
```

#### Entrypoint
The tool may be specified as the `ENTRYPOINT` of a docker image to allow for
easy use of AWS SSM Parameter Store parameters with your application:
```Dockerfile
FROM alpine

# install the binary release using one of the methods above

# make the binary executable
RUN chmod +x /pse

# specify AWS_REGION unless passed from outside your container
ENV AWS_REGION=eu-west-1

# use an ENV var to specify the parameter(s) to use
ENV PARAMETER_PATH=/some/path
ENTRYPOINT /pse "$PARAMETER_PATH" -- env
# -- or --
# specify the parameter(s) directly in the ENTRYPOINT
# and optionally use CMD
ENTRYPOINT [ "/pse", "/some/path", "--" ]
CMD ["env"]
```


## Contributing
1. Fork it (<https://github.com/neochrome/parameter-store-executor/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Releasing
To cut a new release, push a tag with the new (semver) version, e.g: `v1.2.3`.
The tag should refer to a commit that has the `shard.yml` version and `./src/version.cr`
updated with the new version. The script `./scripts/version` synchronizes this
and also perform some checks:
1. Detect uncommitted changes
2. Check code formatting
3. Passing tests
4. Sanity check the new version

# Parameter Store Executor

Fetches parameters recursively at PARAMETER_PATHs from AWS SSM Parameter Store.  
Then executes CMD with the parameters transformed into ENVIRONMENT variables.

The parameter names will be transformed as:
 - Make relative to the corresponding PARAMETER_PATH
 - Replace all '/' & '-' characters with '_'
 - Make UPPERCASE

Conflicting parameters will resolve to the value of the last one found.
```gherkin
Given the following parameters:
| name      | value |
| /one/test | 1     |
| /two/test | 2     |

When requesting: [/, /one, /two]

Then the following ENVIRONMENT variables will be available:
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
TODO: Describe

## Usage
```sh
pse /path/to/parameters -- env
```

### In Docker image
TODO: Describe

## Contributing

1. Fork it (<https://github.com/neochrome/parameter-store-executor/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

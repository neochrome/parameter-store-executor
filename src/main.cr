require "option_parser"
require "./version"
require "log"

Log.setup_from_env default_level: :error

paths = [] of String
cmd = [] of String
clean_env = false

OptionParser.parse do |parser|
  parser.on "-h", "--help", "Display this help and exit" do
    puts <<-HELP
    Parameter Store Executor

    usage: pse [OPTIONS] PARAMETER_PATH... [-- COMMAND [ARGS]]

    Fetches parameters recursively at PARAMETER_PATHs from AWS SSM Parameter Store.
    Then executes CMD with the parameters transformed into ENV variables.

    Options:
    #{parser}

    The parameter names will be transformed as:
     - Make relative to the corresponding PARAMETER_PATH
     - Replace all '/' & '-' characters with '_'
     - Make UPPERCASE

    Conflicting parameters will resolve to the value of the last one found.
    Any existing ENV variables (unless --clean-env is specified) will be passed
    along and takes precedence over parameters with the same name - to allow
    overriding specific parameters (e.g in development environment).

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
    HELP
    exit
  end

  parser.on "-v", "--version", "Display version and exit" do
    puts "PSE version #{VERSION}"
    exit
  end

  parser.on "--clean-env", "Don't pass any existing ENV variables" do
    clean_env = true
  end

  parser.on "--log-level=LOG_LEVEL", "Set the logging level (#{Log::Severity.names.join("|")}) (default error)" do |level|
    unless Log::Severity.parse? level
      STDERR.puts "Invalid log level: #{level}"
      exit 1
    end
    Log.setup(Log::Severity.parse(level))
  end

  parser.unknown_args do |args|
    if args.empty?
      STDERR.puts "At least one PARAMETER_PATH must be specified."
      exit 1
    end
    paths = args
    cmd = ARGV.skip paths.size
  end

  parser.invalid_option do |option|
    STDERR.puts "Unknown option: #{option}"
    STDERR.puts "Try: pse --help"
    exit 1
  end

  parser.missing_option do |option|
    STDERR.puts "Missing value for option: #{option}"
    STDERR.puts "Try: pse --help"
    exit 1
  end
end

require "aws-credentials"
include Aws::Credentials
provider = Providers.new ([
  EnvProvider.new,
  SharedCredentialFileProvider.new,
  InstanceMetadataProvider.new,
] of Provider)

require "./ssm"
ssm = SSM.new provider.credentials

require "./env_vars"
begin
  env = paths.reduce(EnvVars.new) do |vars, path|
    parameters = ssm.get_parameters_by_path path
    parameters.each do |p|
      Log.info &.emit("parameter", name: p.name)
    end
    vars.merge EnvVars.from_parameters parameters
  end
  env.merge! ENV.to_h unless clean_env

  if cmd.empty?
    env.each { |k, v| puts "export #{k}=#{Process.quote(v)}" }
    exit
  end

  Process.exec(cmd.first, args: cmd.skip(1), env: env, clear_env: true, shell: true)
rescue ex
  Log.debug(exception: ex) { "" }
  Log.error { ex.message }
  exit 1
end

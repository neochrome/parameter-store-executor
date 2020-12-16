require "option_parser"
require "./version"
require "log"

Log.setup_from_env default_level: :error

paths = [] of String
cmd = [] of String
clear_env = false

OptionParser.parse do |parser|

  parser.on "-h", "--help", "Display this help and exit", do
    puts "Parameter Store Executor"
    puts ""
    puts "usage: pse [OPTIONS] PARAMETER_PATH... [-- COMMAND [ARGS]]"
    puts ""
    puts "Fetches parameters recursively at PARAMETER_PATHs from AWS SSM Parameter Store."
    puts "Then executes CMD with the parameters transformed into ENVIRONMENT variables."
    puts ""
    puts "Options:"
    puts parser
    puts ""
    puts "The parameter names will be transformed as:"
    puts " - Make relative to the corresponding PARAMETER_PATH"
    puts " - Replace all '/' & '-' characters with '_'"
    puts " - Make UPPERCASE"
    puts ""
    puts "Conflicting parameters will resolve to the value of the last one found."
    puts ""
    puts "Example:"
    puts ""
    puts "Given the parameters:"
    puts " /one/test => '1'"
    puts " /two/test => '2'"
    puts "When requesting: / /one /two"
    puts "Then the following ENVIRONMENT variables will be available:"
    puts " ONE_TEST => '1'"
    puts " TWO_TEST => '2'"
    puts " TEST => '2'"
    exit
  end

  parser.on "-v", "--version", "Display version and exit" do
    puts "PSE version #{VERSION}"
    exit
  end

  parser.on "--clear-env", "Don't pass any existing ENV variables" do
    clear_env = true
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
    vars.merge EnvVars.from_parameters ssm.get_parameters_by_path path
  end
  # parameters = paths.map{|p|ssm.get_parameters_by_path p}.flatten
  # parameters.each do |p|
  #   Log.debug &.emit("parameter", name: p.name)
  # end
  # env = EnvVars.from_parameters parameters

  if cmd.empty?
    env.each { |k,v| puts "#{k}=#{v}" }
    exit
  end

  Process.exec(cmd.first, args: cmd.skip(1), env: env, clear_env: clear_env, shell: true)

rescue ex
  Log.debug(exception: ex) { "" }
  Log.error { ex.message }
  exit 1
end

require "aws-credentials"
require "awscr-signer"
require "http/client"
require "http/params"
require "uri"
require "json"
require "log"
require "path"

class SSM
  Log = ::Log.for self

  struct Parameter
    include JSON::Serializable
    @[JSON::Field(key: "Name")]
    getter name : String
    @[JSON::Field(key: "Value")]
    getter value : String
    # not used
    @ARN : String?
    @DataType : String?
    @LastModifiedDate : Float32?
    @Selector : String?
    @SourceResult : String?
    @Type : String?
    @Version : Int64?

    def initialize(@name, @value)
    end

    def <=>(other : self)
      name <=> other.name
    end

    def relative(path : String)
      Parameter.new name: Path[name].relative_to(path).to_s, value: value
    end
  end

  struct Response
    include JSON::Serializable
    @[JSON::Field(key: "NextToken")]
    getter next_token : String?
    @[JSON::Field(key: "Parameters")]
    getter parameters = [] of SSM::Parameter

    def initialize
    end
  end

  struct Error
    include JSON::Serializable
    getter message : String
  end

  def initialize(
    creds : Aws::Credentials::Credentials,
    @region : String = ENV["AWS_REGION"]
  )
    @client = HTTP::Client.new "ssm.#{@region}.amazonaws.com", tls: true
    signer = Awscr::Signer::Signers::V4.new(
      "ssm",
      @region,
      creds.access_key_id,
      creds.secret_access_key,
      creds.session_token
    )
    @client.before_request do |request|
      signer.sign request, add_sha: false
      Log.trace &.emit("signed", headers: request.headers.to_h)
    end
  end

  private def send(operation, body)
    @client.post(
      "/",
      headers: HTTP::Headers{
        "User-Agent"   => "pse",
        "Content-Type" => "application/x-amz-json-1.1",
        "X-Amz-Target" => "AmazonSSM.#{operation}",
      },
      body: body
    )
  end

  def get_parameters_by_path(path : String) : Array(Parameter)
    parameters = [] of Parameter
    response = Response.new
    loop do
      Log.info &.emit("get_parameters_by_path", path: path)
      body = JSON.build do |json|
        json.object do
          json.field "Path", path
          json.field "Recursive", true
          json.field "WithDecryption", true
          json.field "NextToken", response.next_token unless response.next_token.nil?
        end
      end
      res = send "GetParametersByPath", body
      if res.success?
        response = Response.from_json res.body
        parameters += response.parameters
      else
        error = Error.from_json res.body
        raise error.message
      end
      break if response.next_token.nil?
    end
    parameters.sort.map &.relative path
  end
end

require "./ssm"

class EnvVars < Hash(String, String)
  def self.from_hash(parameters : Hash(String, String)) : EnvVars
    e = EnvVars.new
    parameters
      .transform_keys(&.upcase.tr(from: "/-", to: "_"))
      .each { |k, v| e[k] = v }
    e
  end

  def self.from_parameters(parameters : Array(SSM::Parameter)) : EnvVars
    from_hash(
      parameters.to_h { |p| {p.name, p.value} }
    )
  end
end

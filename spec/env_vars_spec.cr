require "./spec_helper"
require "../src/env_vars"

describe EnvVars do
  it "is empty when created" do
    EnvVars.new.empty?.should be_true
  end

  context "#from_hash" do
    it "is empty when created with parameters" do
      p = {} of String => String
      e = EnvVars.from_hash p
      e.empty?.should be_true
    end

    it "makes parameter name UPPERCASE" do
      EnvVars.from_hash({
        "one" => "1",
      }).should eq EnvVars{
        "ONE" => "1",
      }
    end

    it "replaces /" do
      EnvVars.from_hash({
        "one/two" => "12",
      }).should eq EnvVars{
        "ONE_TWO" => "12",
      }
    end

    it "replaces -" do
      EnvVars.from_hash({
        "one-two" => "12",
      }).should eq EnvVars{
        "ONE_TWO" => "12",
      }
    end

    it "handles several parameters" do
      EnvVars.from_hash({
        "one"           => "1",
        "one/two"       => "12",
        "one/two-three" => "123",
      }).should eq EnvVars{
        "ONE"           => "1",
        "ONE_TWO"       => "12",
        "ONE_TWO_THREE" => "123",
      }
    end
  end

  context "#merge" do
    it "is same when merged with empty" do
      EnvVars.from_hash({
        "one/two" => "12",
      })
        .merge(EnvVars.new)
        .should eq EnvVars{
          "ONE_TWO" => "12",
        }
    end

    it "overwrites keys with same name" do
      EnvVars{
        "ONE"     => "1",
        "ONE_TWO" => "one2",
      }.merge(EnvVars{
        "ONE_TWO" => "12",
        "THREE"   => "3",
      }).should eq EnvVars{
        "ONE"     => "1",
        "ONE_TWO" => "12",
        "THREE"   => "3",
      }
    end
  end
end
